import { useParams, useSearchParams } from "react-router-dom";
import { useSaveDesignSystem } from "./DesignSystemQueries";
import styles from "./SidebarDesignSystem.module.css";
import SidebarSection from "./SidebarSection";
import IconColors from "../../ui/icons/IconColors";
import SidebarFolder from "./SidebarFolder";
import { useDesignSystemContext } from "./DesignSystemContext";
import SidebarFile from "./SidebarFile";
import {
  MdAbc,
  MdEdit,
  MdSave,
  MdSettings,
  MdTextFields,
  MdVisibility,
} from "react-icons/md";
import { ICON_SIZE_MD, ICON_SIZE_SM } from "../../ui/UiConstants";
import Popover from "../../ui/kit/Popover";
import SidebarSettings from "./SidebarSettings";
import Switch from "../../ui/kit/Switch";
import { invoke } from "@tauri-apps/api";
import { RecentFile } from "../../domain/HomeDomain";
import toast from "react-hot-toast";
import { useQueryClient } from "@tanstack/react-query";
import PaletteIcon from "../../ui/icons/PaletteIcon";
import BaseIcon from "../../ui/icons/BaseIcon";
import { useBaseColors } from "../../util/DesignSystemUtils";
import ThemeIcon from "../../ui/icons/ThemeIcon";
import FontIcon from "../../ui/icons/FontIcon";

function SidebarDesignSystem() {
  const { designSystem, setActiveComponent } = useDesignSystemContext();
  const { designSystemPath } = useParams();
  const { saveDesignSystem } = useSaveDesignSystem(designSystemPath);
  const queryClient = useQueryClient();
  function handleSave() {
    saveDesignSystem({ designSystem, isTmp: false });
  }

  const base = useBaseColors();

  const [searchParams, setSearchParams] = useSearchParams();
  const editMode: boolean = JSON.parse(
    searchParams.get("editMode") || "false"
  ) as boolean;

  const VisibilityIcon = editMode ? MdEdit : MdVisibility;

  function toggleSearchParams() {
    const newEditMode = !editMode;
    searchParams.set("editMode", String(newEditMode));
    setSearchParams(searchParams);
    //isEditMode is active when we toggle => then switch to read only
    if (!newEditMode) {
      setActiveComponent({
        componentId: "",
        mode: "default",
      });
    }

    if (!designSystemPath) return;
    invoke<{ updatedFile: RecentFile }>("update_recent_file", {
      updatedFile: {
        filePath: designSystemPath,
        editMode: newEditMode,
      },
    })
      .then(() => {
        queryClient.invalidateQueries({
          queryKey: "recent-files",
        });
      })
      .catch((err) => {
        toast.error(`Fail to save read only : ${err}`);
      });
  }

  return (
    <div className={styles.sidebarDesignSystem}>
      <div className={styles.topContainer}>
        <div className={styles.topMenu}>
          <Popover>
            <Popover.Toggle id="settings">
              <button className="action-ghost-button">
                <MdSettings size={ICON_SIZE_MD} />
              </button>
            </Popover.Toggle>
            <Popover.Body id="settings">
              <SidebarSettings />
            </Popover.Body>
          </Popover>
          <button
            className="action-ghost-button"
            onClick={handleSave}
            disabled={!designSystem.metadata.isTmp}
          >
            <MdSave size={ICON_SIZE_MD} />
          </button>
        </div>
        <SidebarSection
          SectionIcon={IconColors}
          name="Colors"
          scrollName="colors"
        >
          <>
            <SidebarFolder name="Palettes">
              {designSystem?.palettes.map((palette) => (
                <SidebarFile
                  key={palette.paletteName}
                  filename={palette.paletteName}
                  underFolder={true}
                  id={`palette-${palette.paletteName}`}
                  icon={<PaletteIcon palette={palette} size={ICON_SIZE_SM} />}
                />
              ))}
            </SidebarFolder>
            <SidebarFolder name="Base">
              <SidebarFile
                filename="Base"
                id="base"
                underFolder={true}
                icon={<BaseIcon base={base} size={ICON_SIZE_SM} />}
              />
            </SidebarFolder>
            <SidebarFolder name="Themes">
              {designSystem?.themes.map((theme) => (
                <SidebarFile
                  key={theme.themeName}
                  filename={theme.themeName}
                  underFolder={true}
                  id={`theme-${theme.themeName}`}
                  icon={<ThemeIcon size={ICON_SIZE_SM} theme={theme} />}
                />
              ))}
            </SidebarFolder>
          </>
        </SidebarSection>
        <SidebarSection SectionIcon={FontIcon} name="Texts" scrollName="texts">
          <>
            <SidebarFile
              filename="Fonts"
              id="fonts"
              underFolder={true}
              icon={<MdAbc size={ICON_SIZE_SM} />}
            />
            <SidebarFile
              filename={"Typography"}
              underFolder={true}
              id={"typography"}
              icon={<MdTextFields size={ICON_SIZE_SM} />}
            />
          </>
        </SidebarSection>
      </div>
      <div className={styles.bottomContainer}>
        <div className="row align-center gap-2">
          <Switch checked={editMode} onChange={toggleSearchParams} />
          <VisibilityIcon size={ICON_SIZE_SM} />
        </div>
      </div>
    </div>
  );
}

export default SidebarDesignSystem;
