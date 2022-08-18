export class Editor {
    constructor() {
        this.editor = null;
    }

    init(element_id) {
        this.editor = new SimpleMDE({
            element: document.getElementById("editor"),
            status: false,
            toolbar: false,
            toolbarTips: false,
        });
    }
}

// simplemde.togglePreview();

