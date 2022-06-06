import "./App.css";

import AppBar from "@mui/material/AppBar";
import Paper from "@mui/material/Paper";
import Stack from "@mui/material/Stack";
import { styled } from "@mui/material/styles";
import Toolbar from "@mui/material/Toolbar";
import * as React from "react";

import ChoiceButton from "./components/choice-button";

const Item = styled(Paper)(({ theme }) => ({
  backgroundColor: theme.palette.mode === "dark" ? "#1A2027" : "#fff",
  ...theme.typography.body1,
  padding: theme.spacing(1),
  textAlign: "center",
  color: theme.palette.text.secondary,
}));

function App() {
  return (
    <div className="App">
      <AppBar
        style={{
          textAlign: "center",
        }}
      >
        <Toolbar>React Voting System</Toolbar>
      </AppBar>
      <p>Solana Voting System</p>
      <ChoiceButton />
      {/* <div>
        <Stack className="vote-stack" direction="row" spacing={8}>
          <Item className="vote-box">Item 1</Item>
          <Item className="vote-box">Item 2</Item>
        </Stack>
      </div> */}
    </div>
  );
}

export default App;
