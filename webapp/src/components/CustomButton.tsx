import { Button } from '@mui/material';
import { styled } from '@mui/system';

const GradientButton = styled(Button)({
  background: 'linear-gradient(45deg, #FE6B8B 30%, #FF8E53 90%)',
  border: 0,
  borderRadius: 3,
  boxShadow: '0 3px 5px 2px rgba(255, 105, 135, .3)',
  color: 'white',
  height: 48,
  padding: '0 30px',
});

// @ts-ignore
const CustomButton = ({ children, ...props }) => {
  return <GradientButton {...props}>{children}</GradientButton>;
};

export default CustomButton;