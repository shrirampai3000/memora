import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { ExternalLinkIcon } from "lucide-react";
import { create } from "zustand";
import { commands } from "@/lib/utils/tauri";

export function LoginDialog() {
  const { isOpen, setIsOpen } = useLoginDialog();

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>login required</DialogTitle>
          <DialogDescription>
            please login to continue. you will be redirected to github.com/shrirampai3000/memora
          </DialogDescription>
        </DialogHeader>
        <div className="flex justify-end">
          <Button
            variant="default"
            onClick={() => {
              commands.openLoginWindow(null, null);
              setIsOpen(false);
            }}
          >
            login <ExternalLinkIcon className="w-4 h-4 ml-2" />
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}

interface LoginDialogState {
  isOpen: boolean;
  setIsOpen: (open: boolean) => void;
  checkLogin: (user: any | null, showDialog?: boolean) => boolean;
}

export const useLoginDialog = create<LoginDialogState>((set) => ({
  isOpen: false,
  setIsOpen: (open) => set({ isOpen: open }),
  // Memora runs fully local/BYO-key — there is no required Memora Cloud
  // account, so login never gates a feature and the pop-up is never forced.
  // Sign-in is still available on demand (Account settings' "Log in"), it
  // just isn't a prerequisite for anything. Callers that used the return
  // value to gate now always proceed.
  checkLogin: () => true,
}));
