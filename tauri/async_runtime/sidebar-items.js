window.SIDEBAR_ITEMS = {"enum":[["JoinHandle","An owned permission to join on a task (await its termination)."],["Runtime","A runtime used to execute asynchronous tasks."],["RuntimeHandle","A handle to the async runtime"]],"fn":[["block_on","Runs a future to completion on runtime."],["channel","Creates a bounded mpsc channel for communicating between asynchronous tasks with backpressure."],["handle","Returns a handle of the async runtime."],["set","Sets the runtime to use to execute asynchronous tasks. For convenience, this method takes a [`TokioHandle`]. Note that you cannot drop the underlying [`TokioRuntime`]."],["spawn","Spawns a future onto the runtime."],["spawn_blocking","Runs the provided function on an executor dedicated to blocking operations."]],"struct":[["Mutex","An asynchronous `Mutex`-like type."],["Receiver","Receives values from the associated `Sender`."],["RwLock","An asynchronous reader-writer lock."],["Sender","Sends values to the associated `Receiver`."],["TokioHandle","Handle to the runtime."],["TokioJoinHandle","An owned permission to join on a task (await its termination)."],["TokioRuntime","The Tokio runtime."]]};