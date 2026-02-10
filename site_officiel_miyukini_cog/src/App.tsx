import { Routes, Route } from 'react-router-dom';
import { Layout } from './components/Layout';
import { Home } from './pages/Home';
import { Onboarding } from './pages/Onboarding';
import { Docs } from './pages/Docs';
import { Cores } from './pages/Cores';
import { Services } from './pages/Services';
import { MockCog } from './pages/MockCog';

function App() {
  return (
    <Layout>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/onboarding" element={<Onboarding />} />
        <Route path="/docs" element={<Docs />} />
        <Route path="/cores" element={<Cores />} />
        <Route path="/services" element={<Services />} />
        <Route path="/demo" element={<MockCog />} />
      </Routes>
    </Layout>
  );
}

export default App;
