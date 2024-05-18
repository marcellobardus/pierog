import { useState } from 'react';
import { useRouter } from 'next/router';
import { TextField, Box } from '@mui/material';
import { styled } from '@mui/system';
import CustomButton from '../components/CustomButton';

export const GradientBackground = styled(Box)({
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
  height: '100vh',
  background: 'linear-gradient(to right, #ff7e5f, #feb47b)',
});

const SearchBar = () => {
  const [query, setQuery] = useState('');
  const router = useRouter();

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    if (query.trim()) {
      router.push(`/search?q=${query}`);
    }
  };

  return (
    <GradientBackground>
    <form onSubmit={handleSearch} style={{ width: '50%' }}>
      <TextField
        fullWidth
        variant="outlined"
        placeholder="Search for a Cairo program hash..."
        value={query}
        onChange={(e) => setQuery(e.target.value)}
        InputProps={{
          style: { backgroundColor: 'white', borderRadius: '4px' },
        }}
      />
      <center>
        <CustomButton type="submit" style={{ marginTop: '20px' }}>
          Search
        </CustomButton>
      </center>
    </form>
  </GradientBackground>
  );
};

export default SearchBar;