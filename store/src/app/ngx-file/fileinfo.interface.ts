export interface FileInfo {
    id: number;
    name: string;
    identifier: string;
    size: number;
    mime_type: string;
    created_at: string;
    updated_at: string;
}

export interface FileInfoUpload {
    name: string;
    identifier: string;
    size: number;
    mime_type: string;
    tags: string[];
}
