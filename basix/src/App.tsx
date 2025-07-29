import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import EventDemo from "./components/event";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [user, setUser] = useState("");

  // const invo = window.__TAURI__.core.invoke;

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }


  // calling tauri from frontend
  const getUser = async () => {
    const user = await invoke("get_user", { username: "idityage" });
    console.log(JSON.stringify(user, null, 2));
    setUser(JSON.stringify(user, null, 2));
  }

  const doLogin = async () => {
    const error = await invoke("login", {
      username: "idityage",
      password: "password123",
      age: 20
    })
    console.log(typeof error);
    if (error) {
      console.error("Login failed:", error);
      return;
    }
    console.log("Login successful");
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>

      <button onClick={doLogin}>Login</button>
      <p>Click the button above to login</p>

      <form
        onSubmit={(e) => {
          e.preventDefault();
          getUser();
        }}
      >
        {/* <input
          id="get-user-input"
          type="text"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name to get user..."
        /> */}
        <button type="submit">Get User</button>
      </form>
      <p>{user}</p>

      <br />

      <EventDemo />

    </main>
  );
}

export default App;
