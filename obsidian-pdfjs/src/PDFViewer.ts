import { ItemView, WorkspaceLeaf } from 'obsidian';

export class PDFViewer extends ItemView {
    private iframe: HTMLIFrameElement;
    private pdfjsPath: string;

    constructor(leaf: WorkspaceLeaf, pdfjsPath: string) {
        super(leaf);
        this.pdfjsPath = pdfjsPath;
    }

    getViewType(): string {
        return 'pdf-viewer';
    }

    getDisplayText(): string {
        return 'PDF Viewer';
    }

    async onOpen(): Promise<void> {
        const container = this.containerEl.children[1] as HTMLElement;
        container.empty();
        container.style.height = '100%';

        this.iframe = container.createEl('iframe', {
            attr: {
                style: 'width: 100%; height: 100%; border: none;',
                src: this.pdfjsPath
            }
        });
    }

    async onClose(): Promise<void> {
        // Clean up if needed
    }

    async loadPDF(pdfPath: string): Promise<void> {
        if (this.iframe) {
            const viewerUrl = `${this.pdfjsPath}?file=${encodeURIComponent(pdfPath)}`;
            this.iframe.src = viewerUrl;
        }
    }
} 