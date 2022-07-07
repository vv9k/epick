import Foundation

let sha256_un_parsed: String = ProcessInfo.processInfo.environment["SHA"] ?? ""
let chars = Array(sha256_un_parsed)
var sha256: String = ""
for char in chars {
    if char == " " {
        break
    } else {
        sha256.append(char)
    }
}
print(sha256)
