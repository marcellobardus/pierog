import '../styles/global.css';
import '../styles/SearchBar.module.css';
import '../styles/SearchResults.module.css';

import type { AppProps /*, AppContext */ } from 'next/app'

function Home({ Component, pageProps }: AppProps) {
  return <>
      <div className="home">
      <h1>ProgramTracer - Fetch the source code of any program hash</h1>
      <Component {...pageProps} />
    </div>
  </>
}

export default Home;