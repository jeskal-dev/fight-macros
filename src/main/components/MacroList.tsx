import { MacroDto } from '@/main/domain/dtos/macro';
import { generateID } from '@/shared/helpers/generateID';
import { useGlobalActions } from '@/main/hooks/useGlobalActions';
import { Play, Plus } from 'lucide-react';
import { useMemo } from 'react';
import { Show } from '../../shared/components/Conditional';
import { MacroForm } from './MacroForm';
import { MacroItem } from './MacroItem';
import { SectionHeader } from './SectionHeader';
import { SmartList } from '../../shared/components/SmartList';
import { Button } from '../../shared/components/ui/button';
import { ScrollArea } from '../../shared/components/ui/scroll-area';
import { useGlobalState } from '../stores/GlobalStore';
import { useMacroFormActions } from '../stores/MacroFormStore';

export function MacroList() {
  const { state } = useGlobalState();
  const { addMacro, deleteMacro, selectMacro } = useGlobalActions();
  const { profiles, selectedProfileId, selectedMacroId } = state;
  const { setOpen, setValue, setCurrentProfileId } = useMacroFormActions();

  const handleDeleteMacro = (macroId: number) => {
    if (selectedProfile) {
      deleteMacro(selectedProfile.id, macroId);
    }
  };

  const handleDuplicateMacro = (macro: MacroDto) => {
    if (selectedProfile) {
      const duplicatedMacro = {
        id: generateID(),
        name: `${macro.name} (copia)`,
        triggerKey: macro.triggerKey,
        sequence: [...macro.sequence],
      };
      addMacro(selectedProfile.id, duplicatedMacro);
    }
  };

  const selectedProfile = useMemo(
    () => profiles.find((p) => p.id === selectedProfileId),
    [profiles, selectedProfileId]
  );

  const handleAddMacro = () => {
    if (selectedProfile) {
      setOpen(true);
      setValue(null);
      setCurrentProfileId(selectedProfile.id);
    }
  };

  if (!selectedProfile) return null;

  return (
    <div className="h-full bg-background flex flex-col">
      <SectionHeader
        title={selectedProfile?.name || 'Macros'}
        subtitle={
          selectedProfile &&
          `${selectedProfile.macros.length} macros configuradas`
        }
        actions={
          selectedProfile && (
            <Button
              size="sm"
              variant="ghost"
              className="h-6 w-6 p-0"
              onClick={handleAddMacro}
            >
              <Plus className="h-3 w-3" />
            </Button>
          )
        }
      />
      <Show when={selectedProfile?.macros.length === 0}>
        <div className="flex flex-col items-center justify-center h-full text-center">
          <Play className="h-8 w-8 text-muted-foreground mb-2" />
          <p className="text-sm text-muted-foreground">
            No hay macros configuradas
          </p>
          <Button
            size="sm"
            variant="ghost"
            className="mt-2"
            onClick={handleAddMacro}
          >
            <Plus className="h-3 w-3 mr-1" />
            Crear primera macro
          </Button>
        </div>
      </Show>

      <ScrollArea className="flex-1 p-4">
        <Show when={selectedProfile?.macros.length !== 0}>
          <div className="space-y-2 max-h-[calc(100vh-200px)]">
            <SmartList data={selectedProfile?.macros} keyFn={(m) => m.id}>
              {(macro) => (
                <MacroItem
                  key={macro.id}
                  macro={macro}
                  isSelected={selectedMacroId === macro.id}
                  onSelect={() => selectMacro(macro.id)}
                  onEdit={() => {
                    setOpen(true);
                    setValue(macro);
                    setCurrentProfileId(selectedProfile.id);
                  }}
                  onDelete={() => handleDeleteMacro(macro.id)}
                  onDuplicate={() => handleDuplicateMacro(macro)}
                />
              )}
            </SmartList>
          </div>
        </Show>
      </ScrollArea>

      {/* Modal para agregar macro */}
      <MacroForm />
    </div>
  );
}
