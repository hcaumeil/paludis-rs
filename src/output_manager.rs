use cxx::SharedPtr;

use super::bindings::paludis_output_manager_flush;
use super::bindings::paludis_output_manager_succeeded;

pub struct OutputManager {
    pub(crate) ptr: SharedPtr<u64>,
}

impl OutputManager {
    /// Called if an action succeeds. This can be used to, for example, unlink the files behind a to-disk logged output manager.  
    /// If an [OutputManager] is destroyed without having had this method called, it should assume failure. This might mean keeping rather than removing log files, for example.  
    /// Further messages and output may occur even after a call to this method.  
    /// Calls to this method are done by the caller, not by whatever carries out the action in question.  
    /// If ignore_succeeded() has previously been called, does nothing.  
    pub fn succeeded(&mut self) {
        paludis_output_manager_succeeded(self.ptr.clone());
    }

    /// Clients may call this method every few seconds when running multiple processes.  
    /// This is used to display ongoing buffered messages without mixing output from multiple processes.
    pub fn flush(&mut self) {
        paludis_output_manager_flush(self.ptr.clone());
    }
}

pub(crate) fn new_output_manager(ptr: SharedPtr<u64>) -> OutputManager {
    OutputManager { ptr }
}
