import axios from 'axios';
import type { NextApiRequest, NextApiResponse } from 'next';

const handler = async (req: NextApiRequest, res: NextApiResponse) => {
  const { q } = req.query;

  const response = await axios.get(`http://127.0.0.1:4000/search`, {
    params: { program_hash: q }
  });

  if (!response || response.status === 404) {
    return res.status(200).json({ results: [] });
  }

  res.status(200).json({ results: [q] });
};

export default handler;