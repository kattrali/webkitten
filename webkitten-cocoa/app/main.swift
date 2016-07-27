import Cocoa

func log(data: NSData) {
    if let message = NSString(data: data, encoding: NSUTF8StringEncoding) {
        NSLog("%@", message)
    }
}

let task = NSTask()
let bundle = NSBundle.mainBundle()
task.launchPath = bundle.pathForResource("webkitten-cocoa", ofType: nil)
task.environment = ["RUST_BACKTRACE": "1"]

let stdOut = NSPipe()
let stdErr = NSPipe()

stdOut.fileHandleForReading.readabilityHandler = { log($0.availableData) }
stdErr.fileHandleForReading.readabilityHandler = { log($0.availableData) }

task.standardOutput = stdOut
task.standardError = stdErr

task.terminationHandler = { task in
    task.standardOutput?.fileHandleForReading.readabilityHandler = nil
    task.standardError?.fileHandleForReading.readabilityHandler = nil
}

task.launch()
task.waitUntilExit()