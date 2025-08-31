import '@/styles/global.css';
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from '@/shared/components/ui/resizable';
import { MacroList } from './components/MacroList';
import { ProfileSidebar } from './components/ProfileSidebar';
import { SequenceEditor } from './components/SequenceEditor';

import { GlobalProvider } from './providers/GlobalProvider';
import { Toaster } from '@/shared/components/ui/sonner';

function App() {
  return (
    <GlobalProvider>
      <main className="flex h-screen bg-background">
        <ResizablePanelGroup
          className="h-screen max-w-full"
          direction="horizontal"
        >
          <ResizablePanel defaultSize={20} minSize={15} maxSize={30}>
            <ProfileSidebar />
          </ResizablePanel>
          <ResizableHandle withHandle />
          <ResizablePanel defaultSize={40} minSize={25} maxSize={50}>
            <MacroList />
          </ResizablePanel>

          <ResizableHandle withHandle />

          <ResizablePanel defaultSize={40} minSize={25} maxSize={60}>
            <SequenceEditor />
          </ResizablePanel>
        </ResizablePanelGroup>
      </main>
      <Toaster />
    </GlobalProvider>
  );
}

export default App;
