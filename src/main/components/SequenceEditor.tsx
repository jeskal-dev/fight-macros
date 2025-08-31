import { SequenceStepDto } from '@/main/domain/dtos/sequence';
import { useGlobalActions } from '@/main/hooks/useGlobalActions';
import {
  closestCenter,
  DndContext,
  DragEndEvent,
  DragOverlay,
  DragStartEvent,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
} from '@dnd-kit/core';
import {
  arrayMove,
  SortableContext,
  sortableKeyboardCoordinates,
  verticalListSortingStrategy,
} from '@dnd-kit/sortable';
import { Play, Plus } from 'lucide-react';
import { useMemo, useState } from 'react';
import { Choose, Otherwise, When } from '../../shared/components/Conditional';
import { SectionHeader } from './SectionHeader';
import { SequenceForm } from './SequenceForm';
import { SequenceStepItem } from './SequenceStepItem';
import { Badge } from '../../shared/components/ui/badge';
import { Button } from '../../shared/components/ui/button';
import { useGlobalState } from '../stores/GlobalStore';
import { useSequenceFormActions } from '../stores/SequenceFormStore';
import { ScrollArea } from '@/shared/components/ui/scroll-area';

export function SequenceEditor() {
  const {
    setOpen,
    setValue,
    setCurrentMacroId,
    setSequence,
    setCurrentProfileId,
  } = useSequenceFormActions();
  const { state } = useGlobalState();
  const { updateMacroSequence } = useGlobalActions();
  const [activeId, setActiveId] = useState<number | null>(null);

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  const { profiles, selectedProfileId, selectedMacroId } = state;

  const selectedProfile = useMemo(
    () => profiles.find((p) => p.id === selectedProfileId),
    [profiles, selectedProfileId]
  );
  const selectedMacro = useMemo(
    () => selectedProfile?.macros.find((m) => m.id === selectedMacroId),
    [selectedProfile, selectedMacroId]
  );

  const handleAddStep = () => {
    if (!(selectedProfile && selectedMacro)) return;
    setCurrentProfileId(selectedProfile.id);
    setCurrentMacroId(selectedMacro.id);
    setSequence([...selectedMacro.sequence]);
    setValue(null);
    setOpen(true);
  };

  const handleUpdateStep = (step: SequenceStepDto) => {
    if (!(selectedProfile && selectedMacro)) return;
    setCurrentMacroId(selectedMacro.id);
    setCurrentProfileId(selectedProfile.id);
    setSequence([...selectedMacro.sequence]);
    setValue(step);
    setOpen(true);
  };

  const handleDeleteStep = (stepId: number) => {
    if (selectedProfile && selectedMacro) {
      const newSequence = selectedMacro.sequence.filter(
        (step) => step.id !== stepId
      );
      updateMacroSequence(selectedProfile.id, selectedMacro.id, newSequence);
    }
  };

  const handleDragEnd = (event: DragEndEvent) => {
    console.log('handleDragEnd', event);
    const { active, over } = event;

    if (active.id !== over?.id && selectedProfile && selectedMacro) {
      const oldIndex = selectedMacro.sequence.findIndex(
        (step) => step.id.toString() === active.id
      );
      const newIndex = selectedMacro.sequence.findIndex(
        (step) => step.id.toString() === over?.id
      );

      const newSequence = arrayMove(selectedMacro.sequence, oldIndex, newIndex);
      updateMacroSequence(selectedProfile.id, selectedMacro.id, [
        ...newSequence,
      ]);
    }

    setActiveId(null);
  };

  const handleDragStart = (event: DragStartEvent) => {
    setActiveId(Number(event.active.id));
  };

  const RenderOverlay = () => {
    if (!activeId) return null;
    const step = selectedMacro?.sequence.find((s) => s.id === activeId);
    if (!step) return null;

    return <SequenceStepItem key={step.id} step={step} index={-1} />;
  };

  if (!selectedMacro) {
    return (
      <div className="h-full bg-background flex flex-col">
        <SectionHeader title="Secuencia" />
        <div className="flex-1 flex items-center justify-center text-center">
          <div>
            <Play className="h-8 w-8 text-muted-foreground mx-auto mb-2" />
            <p className="text-sm text-muted-foreground">
              Selecciona un macro para editar su secuencia
            </p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="h-full bg-background flex flex-col">
      <SectionHeader
        title={selectedMacro.name}
        subtitle={
          <>
            Trigger:{' '}
            <Badge variant="outline" className="text-[10px]">
              {selectedMacro.triggerKey}
            </Badge>
            <span className="ml-2">
              • {selectedMacro.sequence.length} pasos
            </span>
          </>
        }
        actions={
          <Button
            size="sm"
            variant="ghost"
            className="h-6 px-2 text-xs"
            onClick={handleAddStep}
          >
            <Plus className="h-3 w-3" />
            Paso
          </Button>
        }
      />

      <ScrollArea className="h-full p-4">
        <div className="   max-h-[calc(100vh-140px)]">
          <Choose>
            <When condition={selectedMacro.sequence.length !== 0}>
              <DndContext
                sensors={sensors}
                collisionDetection={closestCenter}
                onDragStart={handleDragStart}
                onDragEnd={handleDragEnd}
              >
                <SortableContext
                  items={selectedMacro.sequence.map((step) =>
                    step.id.toString()
                  )}
                  strategy={verticalListSortingStrategy}
                >
                  <div className="space-y-2">
                    {selectedMacro.sequence.map((step, index) => (
                      <SequenceStepItem
                        key={step.id}
                        step={step}
                        index={index}
                        onUpdate={() => handleUpdateStep(step)}
                        onDelete={() => handleDeleteStep(step.id)}
                      />
                    ))}
                  </div>
                </SortableContext>

                <DragOverlay>
                  <RenderOverlay />
                </DragOverlay>
              </DndContext>
            </When>
            <Otherwise>
              <div className="flex flex-col items-center justify-center h-full text-center ">
                <Play className="h-8 w-8 text-muted-foreground mb-2" />
                <p className="text-sm text-muted-foreground mb-2">
                  No hay pasos configurados
                </p>
                <Button size="sm" variant="ghost" onClick={handleAddStep}>
                  <Plus className="h-3 w-3 mr-1" />
                  Agregar primer paso
                </Button>
              </div>
            </Otherwise>
          </Choose>
        </div>
      </ScrollArea>

      <SequenceForm />
    </div>
  );
}
