import React, { useState, useEffect, useCallback } from 'react';
import { Karp } from 'karp';

import logo from './logo.svg';
import './App.css';

const TEST_GQL = `
  query GetCity($name: String!) {
    getCityByName(name: $name) {
      id
      name
      weather {
        timestamp
      }
    }
  }
`;

function App() {
  const [karp, setKarp] = useState<Karp>();
  const [loading, setLoading] = useState<any>();
  const loadWasm = async () => {
    try {
      setLoading(true);
      const wasm = await import('karp');
      const client = wasm.Karp.new("https://graphql-weather-api.herokuapp.com/", {
        headers: {
          'test': 'adfadsfsd',
        }
      });
      client.set_headers({
        'ha': 'ba',
      })
      setKarp(client);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadWasm();
  }, []);

  const query = useCallback(
    async () => {
      if (karp) {
        try {
          const data = await karp.query(TEST_GQL, {
            name: 'Sacramento'
          });
          console.log(data);
        } catch (error) {
          console.error(error);
        }
      }
    },
    [karp],
  );

  useEffect(() => {
    if (!loading) {
      query();
    }
  }, [query, loading])

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
