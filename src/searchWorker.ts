
export type SearchResultWorker = {
  indices: number[];
  path: string;
  score: number;
  name: string;
}


let buffer: SearchResultWorker[] = [];

onmessage = (e) => {
  console.log("Recieved Message: ",e.data);
  const { type, payload } = e.data;

  if (type === 'NEW_DATA') {
    buffer.push(payload);
  }

  if (type === 'FLUSH') {
    const chunk = buffer.splice(0, 200);
    postMessage({
      type: 'FLUSHED',
      payload: chunk
    })
  }
}
