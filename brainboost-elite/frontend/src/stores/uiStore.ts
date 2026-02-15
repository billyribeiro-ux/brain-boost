import { create } from 'zustand';

interface Toast {
  id: string;
  message: string;
  type: 'success' | 'error' | 'warning' | 'info';
}

interface Modal {
  id: string;
  component: React.ReactNode;
}

interface UIState {
  theme: 'dark' | 'light';
  toasts: Toast[];
  modals: Modal[];
  sidebarOpen: boolean;
  addToast: (message: string, type: Toast['type']) => void;
  removeToast: (id: string) => void;
  openModal: (id: string, component: React.ReactNode) => void;
  closeModal: (id: string) => void;
  toggleSidebar: () => void;
  toggleTheme: () => void;
}

export const uiStore = create<UIState>((set) => ({
  theme: 'dark',
  toasts: [],
  modals: [],
  sidebarOpen: false,
  addToast: (message, type) =>
    set((state) => ({
      toasts: [
        ...state.toasts,
        { id: Math.random().toString(36), message, type },
      ],
    })),
  removeToast: (id) =>
    set((state) => ({
      toasts: state.toasts.filter((t) => t.id !== id),
    })),
  openModal: (id, component) =>
    set((state) => ({
      modals: [...state.modals, { id, component }],
    })),
  closeModal: (id) =>
    set((state) => ({
      modals: state.modals.filter((m) => m.id !== id),
    })),
  toggleSidebar: () =>
    set((state) => ({
      sidebarOpen: !state.sidebarOpen,
    })),
  toggleTheme: () =>
    set((state) => ({
      theme: state.theme === 'dark' ? 'light' : 'dark',
    })),
}));
