import Foundation

let version: String = ProcessInfo.processInfo.environment["VERSION"] ?? ""
let sha256: String = ProcessInfo.processInfo.environment["SHA"] ?? ""
let url: String = ProcessInfo.processInfo.environment["URL"] ?? ""
let cask = """
cask \"epick\" do
    version \"\(version)\"
    sha256 \"\(sha256)\"

    url \"\(url)\"
    name \"epick\"

    desc \"Color picker for creating harmonic color palettes that works on Linux, Windows, macOS and web.\"
    homepage \"https://github.com/vv9k/epick\"

    app \"app/epick.app\"
end
"""
print(cask)