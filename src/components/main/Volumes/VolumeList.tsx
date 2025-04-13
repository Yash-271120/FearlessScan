import { Volume } from "@/types"
import VolumeComponent from "./VolumeComponent"


type Props = {
  volumes: Volume[]
}
const VolumeList = ({ volumes }: Props) => {
  return (
    <div className="flex flex-row space-x-4 w-full">
      {
        volumes.map((volume, index) => {
          return <VolumeComponent volume={volume} key={index} />
        })
      }
    </div>
  )
}

export default VolumeList;
