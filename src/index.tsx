import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
// With the Tauri API npm package:
import { invoke } from '@tauri-apps/api/tauri'
// Invoke the command
invoke("printToJS");

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root')
);

//doesnt work
// getMatches().then((matches) => {
//     // do something with the { args, subcommand } matches
//     console.log(matches);
//   });
// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
