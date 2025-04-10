import { useState } from "react";
import { toast } from "sonner";


import { invoke } from "@tauri-apps/api/core";
import { Toaster } from "@/components/ui/sonner";
import "./App.css";
import { Volume } from "./types"

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    try {
      // await invoke("greet", { name })
      setGreetMsg(await invoke("greet", { name }));
      await invoke("increase_counter", { add: 10 });
      const msg = await invoke("show_counter");
      const data = await invoke<Volume[]>("get_volumes");
      console.log("Yash; ", msg);

      console.log("Yash:2 ", data)
    } catch (err) {
      console.log("Yash: ", err)
      toast.error(err);
    }
  }

  return (
    <main className="container">
      <h1>Yash here</h1>
      <Toaster richColors />
    </main>
  );
}

export default App;
