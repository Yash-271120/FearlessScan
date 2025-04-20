export interface Volume {
    name: string;
    mountPoint: string;
    availableGb: number;
    usedGb: number;
    totalGb: number;
}

export interface DirectoryPath {
  type: "directory" | "file";
  name: string;
  path: string;
}

export interface SearchResult{
  indices: number[];
  path: string;
  score: number;
  name: string;
}

export interface FSEvent{
  kind: "create" | "remove";
  directoryPath: DirectoryPath; 
}
