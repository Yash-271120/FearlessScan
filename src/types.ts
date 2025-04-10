export interface Volume {
    name: string;
    mountpoint: string;
    available_gb: number;
    used_gb: number;
    total_gb: number;
}
