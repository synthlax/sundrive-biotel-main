import { Suspense, type Component } from 'solid-js';
import { A, useLocation } from '@solidjs/router';
import { Branding } from "./components/branding";
import { BaseHeader } from './components/base-header';

const App: Component<{children}> = (props: { children: Element }) => {
  const location = useLocation();

  return (
    <>
      <BaseHeader></BaseHeader>
      <main>
        <Suspense>{props.children}</Suspense>
      </main>
    </>
  );
};

export default App;
