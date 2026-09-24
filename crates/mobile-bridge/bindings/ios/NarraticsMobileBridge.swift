import Foundation

/// Narratics Native Mobile Bridge for iOS / iPadOS
/// Wraps the high-performance Rust Engine-Core (Tree Move CRDT & Yrs Text Engine)
public final class NarraticsMobileEngine {
    public struct NodeInfo: Codable, Identifiable {
        public let id: String
        public let parentId: String?
        public let rank: String
        public let title: String
        public let isFolder: Bool

        enum CodingKeys: String, CodingKey {
            case id
            case parentId = "parent_id"
            case rank
            case title
            case isFolder = "is_folder"
        }
    }

    public struct SceneInfo: Codable, Identifiable {
        public let id: String
        public let title: String
        public let text: String
        public let wordCount: Int
        public let updatedAt: Int64

        enum CodingKeys: String, CodingKey {
            case id, title, text
            case wordCount = "word_count"
            case updatedAt = "updated_at"
        }
    }

    public struct ProjectState: Codable {
        public let title: String
        public let author: String
        public let path: String
        public let nodes: [NodeInfo]
        public let scenes: [SceneInfo]
        public let loreCount: Int

        enum CodingKeys: String, CodingKey {
            case title, author, path, nodes, scenes
            case loreCount = "lore_count"
        }
    }

    private struct CResponse<T: Codable>: Codable {
        let success: Bool
        let data: T?
        let error: String?
    }

    public let projectPath: String

    public init(projectPath: String) {
        self.projectPath = projectPath
    }

    public func openProject() throws -> ProjectState {
        return try executeCFunction { pathPtr in
            narr_mobile_open(pathPtr)
        }
    }

    public func saveScene(id: String, text: String) throws -> SceneInfo {
        return try executeCFunction3(arg1: projectPath, arg2: id, arg3: text) { p1, p2, p3 in
            narr_mobile_save_scene(p1, p2, p3)
        }
    }

    public func moveNode(childId: String, parentId: String, rank: String, title: String) throws -> [NodeInfo] {
        guard let pPath = projectPath.cString(using: .utf8),
              let pChild = childId.cString(using: .utf8),
              let pParent = parentId.cString(using: .utf8),
              let pRank = rank.cString(using: .utf8),
              let pTitle = title.cString(using: .utf8) else {
            throw NSError(domain: "NarraticsMobile", code: -1, userInfo: [NSLocalizedDescriptionKey: "CString conversion failure"])
        }

        guard let resPtr = narr_mobile_move_node(pPath, pChild, pParent, pRank, pTitle) else {
            throw NSError(domain: "NarraticsMobile", code: -2, userInfo: [NSLocalizedDescriptionKey: "Null pointer returned"])
        }
        defer { narr_mobile_free_string(resPtr) }

        let jsonString = String(cString: resPtr)
        guard let data = jsonString.data(using: .utf8) else {
            throw NSError(domain: "NarraticsMobile", code: -3, userInfo: [NSLocalizedDescriptionKey: "UTF-8 data decode failure"])
        }

        let resp = try JSONDecoder().decode(CResponse<[NodeInfo]>.self, from: data)
        if resp.success, let val = resp.data {
            return val
        } else {
            throw NSError(domain: "NarraticsMobile", code: -4, userInfo: [NSLocalizedDescriptionKey: resp.error ?? "Unknown error"])
        }
    }

    private func executeCFunction<T: Codable>(_ fn: (UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?) throws -> T {
        guard let pathC = projectPath.cString(using: .utf8) else {
            throw NSError(domain: "NarraticsMobile", code: -1, userInfo: [NSLocalizedDescriptionKey: "Invalid path"])
        }
        guard let resPtr = fn(pathC) else {
            throw NSError(domain: "NarraticsMobile", code: -2, userInfo: [NSLocalizedDescriptionKey: "Null pointer returned"])
        }
        defer { narr_mobile_free_string(resPtr) }

        let jsonString = String(cString: resPtr)
        let data = jsonString.data(using: .utf8)!
        let resp = try JSONDecoder().decode(CResponse<T>.self, from: data)
        if resp.success, let val = resp.data {
            return val
        } else {
            throw NSError(domain: "NarraticsMobile", code: -4, userInfo: [NSLocalizedDescriptionKey: resp.error ?? "Unknown error"])
        }
    }

    private func executeCFunction3<T: Codable>(arg1: String, arg2: String, arg3: String, _ fn: (UnsafePointer<CChar>, UnsafePointer<CChar>, UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?) throws -> T {
        guard let c1 = arg1.cString(using: .utf8),
              let c2 = arg2.cString(using: .utf8),
              let c3 = arg3.cString(using: .utf8) else {
            throw NSError(domain: "NarraticsMobile", code: -1, userInfo: [NSLocalizedDescriptionKey: "Encoding error"])
        }
        guard let resPtr = fn(c1, c2, c3) else {
            throw NSError(domain: "NarraticsMobile", code: -2, userInfo: [NSLocalizedDescriptionKey: "Null pointer returned"])
        }
        defer { narr_mobile_free_string(resPtr) }

        let jsonString = String(cString: resPtr)
        let data = jsonString.data(using: .utf8)!
        let resp = try JSONDecoder().decode(CResponse<T>.self, from: data)
        if resp.success, let val = resp.data {
            return val
        } else {
            throw NSError(domain: "NarraticsMobile", code: -4, userInfo: [NSLocalizedDescriptionKey: resp.error ?? "Unknown error"])
        }
    }
}

// C-Declarations
@_silgen_name("narr_mobile_open")
private func narr_mobile_open(_ path: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?

@_silgen_name("narr_mobile_save_scene")
private func narr_mobile_save_scene(_ path: UnsafePointer<CChar>, _ sceneId: UnsafePointer<CChar>, _ text: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?

@_silgen_name("narr_mobile_move_node")
private func narr_mobile_move_node(_ path: UnsafePointer<CChar>, _ child: UnsafePointer<CChar>, _ parent: UnsafePointer<CChar>, _ rank: UnsafePointer<CChar>, _ title: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?

@_silgen_name("narr_mobile_free_string")
private func narr_mobile_free_string(_ ptr: UnsafeMutablePointer<CChar>?)
