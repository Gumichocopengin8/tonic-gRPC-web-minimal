import { useEffect } from 'react';
import { EchoClient } from 'proto-client/EchoServiceClientPb';
import { EchoRequest, type EchoResponse } from 'proto-client/echo_pb';

import './App.css';

function App() {
  useEffect(() => {
    const client = new EchoClient('http://localhost:8000');
    const echoRequest = new EchoRequest();
    echoRequest.setMessage('World');
    client.unaryEcho(echoRequest, null).then((res: EchoResponse) => {
      console.log(res, res.getMessage());
    });

    const stream = client.serverStreamingEcho(echoRequest);
    stream.on('data', (data) => {
      console.log('data: ', data);
    });

    return () => {
      stream.cancel();
    };
  }, []);

  return (
    <>
      <div>test</div>
    </>
  );
}

export default App;
