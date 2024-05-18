import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import axios from 'axios';

import { GradientBackground } from '.';
import Link from 'next/link';
import CustomButton from '@/components/CustomButton';

const Search = () => {
  const router = useRouter();
  const { q } = router.query;
  const [results, setResults] = useState([]);

  useEffect(() => {
    if (q) {
      const fetchResults = async () => {
        try {
          const response = await axios.get(`/api/search?q=${q}`);

          setResults(response.data.results);
        } catch (error) {
          console.error('Error fetching search results:', error);
        }
      };
      fetchResults();
    }
  }, [q]);

  const isMatch = results.length > 0;

  return (
      <GradientBackground>
        <div className="search-results">
        {isMatch ? (
          <>
            <h2>Program hash found!</h2>
          <CustomButton style={{ top: 0 }}>Download Sources</CustomButton>
          </>
      ) : (
        <div>
          <h2>Could not find corresponding source code for this program hash</h2>
          <Link href="/">
            <CustomButton>Back</CustomButton>
          </Link>
        </div>
      )}
        </div>
    </GradientBackground>
  );
};

export default Search;