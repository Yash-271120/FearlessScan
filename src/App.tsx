import { useEffect, useState, useTransition } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import { Toaster } from "@/components/ui/sonner";
import { Progress } from "@/components/ui/progress";
import "@/App.css";
import VolumeList from "@/components/main/Volumes/VolumeList";
import { Volume } from "@/types";
import { useNavigationStore } from "./store/navigation";
import ExplorerComponent from "./components/main/Explorer/ExplorerComponent";
import { readPath } from "./icpc-calls";
import { useDirectoryStore } from "./store/directory";
import { toast } from "sonner";

type IndexingProgressPayload = {
  currentItem: number;
  totalItems: number;
  percentage: number;
};

type IndexingStartedPayload = {
  totalItems: number;
};

type IndexingFinishedPayload = {
  totalProcessed: number;
  elapsedTimeMs: number;
};

type AccumulatingStartedPayload = {
  mountPoint: string;
};

type AccumulatingFinishedPayload = {
  mountPoint: string;
  itemsFound: number;
};

function App() {
  const [volumes, setVolumes] = useState<Volume[]>([]);
  const [isIndexing, setIsIndexing] = useState(false);
  const [isAccumulating, setIsAccumulating] = useState(false);
  const [indexingProgress, setIndexingProgress] = useState(0);
  const [indexingStats, setIndexingStats] = useState<{current: number, total: number}>({current: 0, total: 0});
  const [accumulatingMountPoint, setAccumulatingMountPoint] = useState<string>("");
  const { currentIndex, history } = useNavigationStore();
  const { setDirectory, currentMountPoint } = useDirectoryStore();

  const fetchVolumes = async () => {
    if (volumes.length > 0) {
      return;
    }
    const data = await invoke<Volume[]>("get_volumes");
    setVolumes(data);
  };

  const handleReadCurrentPath = async () => {
    try {
      const currPath = history[currentIndex];
      const data = await readPath(currPath, currentMountPoint);
      setDirectory(data);
    } catch (err: any) {
      toast.error(err);
    }
  };

  let render = 0;
  useEffect(() => {
    if (currentIndex === 0) {
      if (render) {
        return;
      }
      render++;
      fetchVolumes();
      return;
    }

    handleReadCurrentPath();
  }, [currentIndex]);

  useEffect(() => {
    // Listen for accumulating started event
    const unlistenAccumulatingStarted = listen<AccumulatingStartedPayload>("accumulating-started", (event) => {
      setIsAccumulating(true);
      setAccumulatingMountPoint(event.payload.mountPoint);
    });

    // Listen for accumulating finished event
    const unlistenAccumulatingFinished = listen<AccumulatingFinishedPayload>("accumulating-finished", (event) => {
      if (event.payload.mountPoint === "all") {
        setIsAccumulating(false);
      }
    });

    // Listen for indexing started event
    const unlistenIndexingStarted = listen<IndexingStartedPayload>("indexing-started", (event) => {
      setIsIndexing(true);
      setIndexingProgress(0);
      setIndexingStats({current: 0, total: event.payload.totalItems});
    });

    // Listen for indexing progress event
    const unlistenIndexingProgress = listen<IndexingProgressPayload>("indexing-progress", (event) => {
      setIndexingProgress(event.payload.percentage);
      setIndexingStats({
        current: event.payload.currentItem,
        total: event.payload.totalItems
      });
    });
    
    // Listen for indexing finished event
    const unlistenIndexingFinished = listen<IndexingFinishedPayload>("indexing-finished", (event) => {
      setIndexingProgress(100);
      toast.success(`Indexing complete! Processed ${event.payload.totalProcessed.toLocaleString()} files in ${(event.payload.elapsedTimeMs / 1000).toFixed(2)}s`);
      setTimeout(() => setIsIndexing(false), 1000);
    });

    return () => {
      unlistenAccumulatingStarted.then(fn => fn());
      unlistenAccumulatingFinished.then(fn => fn());
      unlistenIndexingStarted.then(fn => fn());
      unlistenIndexingProgress.then(fn => fn());
      unlistenIndexingFinished.then(fn => fn());
    };
  }, []);

  return (
    <main className="bg-black w-screen min-h-screen py-10 px-5">
      {isAccumulating && (
        <div className="fixed top-0 left-0 w-full bg-gray-800 p-4 z-50">
          <div className="text-white mb-2">
            Scanning {accumulatingMountPoint === "all" ? "all volumes" : accumulatingMountPoint} for files...
          </div>
          <Progress value={undefined} className="h-2" /> {/* Indeterminate progress */}
        </div>
      )}
      
      {isIndexing && (
        <div className="fixed top-0 left-0 w-full bg-gray-800 p-4 z-50">
          <div className="text-white mb-2">
            Indexing files: {indexingStats.current.toLocaleString()} of {indexingStats.total.toLocaleString()} ({indexingProgress.toFixed(1)}%)
          </div>
          <Progress value={indexingProgress} className="h-2" />
        </div>
      )}
      
      {currentIndex === 0 ? <VolumeList volumes={volumes} /> : <ExplorerComponent />}
      <Toaster richColors />
    </main>
  );
}

export default App;
