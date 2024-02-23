import axios from 'axios';

const baseURL = 'http://localhost:11434/api/generate';
const baseArgs = { headers: { 'Content-Type': 'application/json' } };

const baseConfig = { 
  "model": "mistral",
  "stream": false
};

const call = async (prompt) => {
  const result = await axios
    .post(
      baseURL,
      { ...baseConfig, prompt },
      baseArgs
    )
  ;

  const { response } = result.data;
  console.log('Response: ', response);

  return response;
}

export default { call };
