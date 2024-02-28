import axios from 'axios'

const baseURL = 'http://localhost:11434/api/generate'
const baseArgs = { headers: { 'Content-Type': 'application/json' } }
const baseConfig = { "model": "codellama:13b", "stream": false }

const call = async (prompt, model = 'codellama:13b') => {
  const result = await axios
    .post(
      baseURL,
      { ...baseConfig, prompt, model },
      baseArgs
    )
  

  const { response } = result.data

  return response
}

export default { call }
