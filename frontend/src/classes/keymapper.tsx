interface HotkeyMapping {
    key: string;
    ctrlKey: boolean;
    shiftKey: boolean;
}

export class KeyMapper {
    private hotkeyMapping: HotkeyMapping;
    private callback: () => void;
    
    constructor(hotkey: HotkeyMapping, callback: () => void) {
        this.hotkeyMapping = hotkey;
        this.handleKeyEvent = this.handleKeyEvent.bind(this);
        this.callback = callback;
    
        // Add event listener to handle key events
        document.addEventListener('keydown', this.handleKeyEvent);
    }

    handleKeyEvent(event: KeyboardEvent): void {
        if (this.isHotkeyDown(event) && event.type === 'keydown') {
            this.callback();
        }
    }
    
    private isHotkeyDown(event: KeyboardEvent): boolean {
        return (
            event.key === this.hotkeyMapping.key &&
            event.ctrlKey === !!this.hotkeyMapping.ctrlKey &&
            event.shiftKey === !!this.hotkeyMapping.shiftKey
        );
    }
    
    private getHotkeyString(): string {
        return `${this.hotkeyMapping.ctrlKey ? 'Ctrl + ' : ''}${this.hotkeyMapping.shiftKey ? 'Shift + ' : ''}${this.hotkeyMapping.key.toUpperCase()}`;
    }

    public toString(): string {
        return this.getHotkeyString();
    }
}