package io.narratics.mobile

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

/**
 * Narratics Native Mobile Bridge for Android
 * Direct JNI bindings to Rust Engine-Core (Tree Move CRDT + Yrs UTF-16)
 */
class NarraticsMobileEngine(private val projectPath: String) {

    init {
        System.loadLibrary("narratics_mobile_bridge")
    }

    @Serializable
    data class NodeInfo(
        val id: String,
        val parent_id: String?,
        val rank: String,
        val title: String,
        val is_folder: Boolean
    )

    @Serializable
    data class SceneInfo(
        val id: String,
        val title: String,
        val text: String,
        val word_count: Int,
        val updated_at: Long
    )

    @Serializable
    data class ProjectState(
        val title: String,
        val author: String,
        val path: String,
        val nodes: List<NodeInfo>,
        val scenes: List<SceneInfo>,
        val lore_count: Int
    )

    @Serializable
    private data class ResponseWrapper<T>(
        val success: Boolean,
        val data: T? = null,
        val error: String? = null
    )

    private val json = Json { ignoreUnknownKeys = true }

    suspend fun openProject(): ProjectState = withContext(Dispatchers.IO) {
        val rawJson = nativeOpen(projectPath)
            ?: throw IllegalStateException("Native bridge returned null")
        val resp = json.decodeFromString<ResponseWrapper<ProjectState>>(rawJson)
        if (resp.success && resp.data != null) {
            resp.data
        } else {
            throw RuntimeException(resp.error ?: "Failed to open project")
        }
    }

    suspend fun saveScene(sceneId: String, text: String): SceneInfo = withContext(Dispatchers.IO) {
        val rawJson = nativeSaveScene(projectPath, sceneId, text)
            ?: throw IllegalStateException("Native bridge returned null")
        val resp = json.decodeFromString<ResponseWrapper<SceneInfo>>(rawJson)
        if (resp.success && resp.data != null) {
            resp.data
        } else {
            throw RuntimeException(resp.error ?: "Failed to save scene")
        }
    }

    suspend fun moveNode(childId: String, parentId: String, rank: String, title: String): List<NodeInfo> = withContext(Dispatchers.IO) {
        val rawJson = nativeMoveNode(projectPath, childId, parentId, rank, title)
            ?: throw IllegalStateException("Native bridge returned null")
        val resp = json.decodeFromString<ResponseWrapper<List<NodeInfo>>>(rawJson)
        if (resp.success && resp.data != null) {
            resp.data
        } else {
            throw RuntimeException(resp.error ?: "Failed to move node")
        }
    }

    // Native JNI bindings
    private external fun nativeOpen(path: String): String?
    private external fun nativeSaveScene(path: String, sceneId: String, text: String): String?
    private external fun nativeMoveNode(path: String, child: String, parent: String, rank: String, title: String): String?
}
