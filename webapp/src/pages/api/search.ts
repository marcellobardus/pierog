import type { NextApiRequest, NextApiResponse } from 'next';

const handler = async (req: NextApiRequest, res: NextApiResponse) => {
  const { q } = req.query;

  // Mock data for demonstration purposes
  const mockData = [
    '0x123'
  ];

  const results = mockData.filter((item) =>
    item.toLowerCase().includes((q as string).toLowerCase())
  );

  res.status(200).json({ results });
};

export default handler;