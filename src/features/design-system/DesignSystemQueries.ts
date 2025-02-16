import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api";
import {
  DesignSystem,
  DesignSystemCreationPayload,
  DesignSystemMetadata,
} from "../../domain/DesignSystemDomain";
import toast from "react-hot-toast";
import { useInsertRecentFile } from "../home/HomeQueries";
import { useParams } from "react-router-dom";
import { useEffect } from "react";

export function useCreateDesignSystem() {
  const { insertRecentFile } = useInsertRecentFile();
  const { mutate: createDesignSystem, isPending: isCreatingDesignSystem } =
    useMutation<DesignSystemMetadata, Error, DesignSystemCreationPayload>({
      mutationFn: async (
        designSystemCreationPayload: DesignSystemCreationPayload
      ) =>
        await invoke("create_design_system", {
          payload: designSystemCreationPayload,
        }),
      onError: (err) => {
        toast.error(err);
      },
      onSuccess: (result: DesignSystemMetadata) => {
        toast.success(
          `Design system ${result.designSystemName} successfully created`
        );
        insertRecentFile(result.designSystemPath);
      },
    });

  return { createDesignSystem, isCreatingDesignSystem };
}

export function useFindDesignSystem(designSystemPath?: string) {
  const { data: designSystem, isLoading: isLoadingDesignSystem } = useQuery({
    queryKey: ["design-system", designSystemPath],
    queryFn: async (): Promise<DesignSystem> =>
      await invoke<DesignSystem>("find_design_system", {
        designSystemPath,
      }),

    enabled: designSystemPath !== undefined && designSystemPath !== null,
  });

  return {
    designSystem,
    isLoadingDesignSystem,
  };
}

export function useCurrentDesignSystem() {
  const { designSystemPath } = useParams();
  const { designSystem, isLoadingDesignSystem } =
    useFindDesignSystem(designSystemPath);

  useEffect(() => {
    document.dispatchEvent(new CustomEvent("refresh-design-system-forms"));
  }, [designSystem]);

  return {
    designSystem,
    isLoadingDesignSystem,
  };
}

//Save temporary
export function useSaveDesignSystem(designSystemPath?: string) {
  const queryClient = useQueryClient();

  const { mutate: saveDesignSystem } = useMutation({
    mutationFn: async ({
      designSystem,
      isTmp,
    }: {
      designSystem: DesignSystem;
      isTmp: boolean;
    }): Promise<DesignSystem> =>
      await invoke("save_design_system", {
        designSystem,
        isTmp,
      }),
    onError: (error) => {
      toast.error(error);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["design-system", designSystemPath],
      });
    },
  });

  return { saveDesignSystem };
}

export function useUndoRedoDesignSystem(designSystemPath?: string) {
  const queryClient = useQueryClient();

  const { mutate: undoDesignSystem } = useMutation({
    mutationFn: async (): Promise<DesignSystem> =>
      await invoke("undo_design_system", {
        designSystemPath,
      }),
    onError: (error) => {
      toast.error(error);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["design-system", designSystemPath],
      });
    },
  });
  const { mutate: redoDesignSystem } = useMutation({
    mutationFn: async (): Promise<DesignSystem> =>
      await invoke("redo_design_system", {
        designSystemPath,
      }),
    onError: (error) => {
      toast.error(error);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["design-system", designSystemPath],
      });
    },
  });

  return { undoDesignSystem, redoDesignSystem };
}
