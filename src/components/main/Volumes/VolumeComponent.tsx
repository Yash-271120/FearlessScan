import { invoke } from "@tauri-apps/api/core";

import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { DirectoryPath, Volume } from "@/types";
import { Progress } from "@/components/ui/progress";
import { useDirectoryStore } from "@/store/directory";
import { toast } from "sonner";
import { useNavigationStore } from "@/store/navigation";

type Props = {
  volume: Volume
}
const VolumeComponent = ({ volume }: Props) => {
  const { setDirectory,setCurrentMountPoint } = useDirectoryStore();
  const { pushToHistory} = useNavigationStore();

  const handleVolumeClick = async () => {
    try {
      const data = await invoke<DirectoryPath[]>("read_directory", {
        path: volume.mountPoint,
        mountPoint: volume.mountPoint,
      })
      setDirectory(data)
      pushToHistory(volume.mountPoint)
      setCurrentMountPoint(volume.mountPoint);
    } catch (err) {
      toast.error(err)
    }
  }

  return (
    <Card className="max-w-lg bg-gray-400 hover:bg-gray-500 border-0 cursor-pointer" onClick={handleVolumeClick}>
      <CardHeader>
        <CardTitle>{volume.name} ({volume.mountPoint})</CardTitle>
      </CardHeader>
      <CardContent>
        <Progress className="bg-white rounded-none" value={(volume.usedGb / volume.totalGb) * 100} max={100} />
        <br />
        {volume.availableGb} GB free of {volume.totalGb} GB
      </CardContent>
    </Card>
  )
}

export default VolumeComponent;
