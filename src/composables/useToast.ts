export interface ToastInstance {
  showToast: (message: string) => void
}

let toastInstance: ToastInstance | null = null

export function useToast() {
  function showToast(message: string) {
    if (toastInstance) {
      toastInstance.showToast(message)
    }
  }

  function registerToast(instance: ToastInstance) {
    toastInstance = instance
  }

  return {
    showToast,
    registerToast
  }
}