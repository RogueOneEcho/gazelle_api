use crate::GazelleError;
use std::future::Future;
use std::panic::resume_unwind;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::shared_clients::SharedClient;
use super::{ExampleValues, get_shared_clients, init_logger};

/// Run a test function for each configured indexer in parallel.
///
/// # Panics
///
/// Panics if no indexers are configured in `config.yml`, or if any test fails.
#[allow(clippy::panic)]
pub async fn for_each_indexer<F, Fut>(test_fn: F) -> Result<(), GazelleError>
where
    F: Fn(String, Arc<Mutex<SharedClient>>, ExampleValues) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), GazelleError>> + Send + 'static,
{
    init_logger();
    let clients = get_shared_clients();
    assert!(
        !clients.is_empty(),
        "No indexers configured. Check config.yml."
    );
    let test_fn = Arc::new(test_fn);
    let tasks: Vec<_> = clients
        .iter()
        .map(|(name, (client, examples))| {
            let test_fn = test_fn.clone();
            let name = name.clone();
            let client = client.clone();
            let examples = examples.clone();
            let task_name = name.clone();
            let handle = tokio::spawn(async move {
                println!("Indexer: {name}");
                test_fn(name, client, examples)
                    .await
                    .map_err(|e| format!("{e:?}"))
            });
            (task_name, handle)
        })
        .collect();

    for (name, task) in tasks {
        match task.await {
            Ok(Ok(())) => {}
            Ok(Err(e)) => panic!("[{name}] {e}"),
            Err(e) if e.is_panic() => resume_unwind(e.into_panic()),
            Err(e) => panic!("[{name}] task failed: {e}"),
        }
    }
    Ok(())
}
