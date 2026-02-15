'use client';

import { useRouter } from 'next/navigation';
import Card from '@/components/ui/Card';
import Button from '@/components/ui/Button';
import { authStore } from '@/stores/authStore';
import { FiLogOut, FiUser, FiMail, FiAward } from 'react-icons/fi';

export default function ProfilePage() {
  const router = useRouter();
  const { user, logout } = authStore();

  const handleLogout = () => {
    logout();
    router.push('/login');
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-textPrimary">Profile</h1>
        <p className="text-textSecondary mt-1">Manage your account settings</p>
      </div>

      <Card>
        <div className="flex items-center gap-4 mb-6">
          <div className="w-20 h-20 rounded-full bg-neuralBlue flex items-center justify-center text-3xl font-bold text-white">
            {user?.display_name?.charAt(0).toUpperCase()}
          </div>
          <div>
            <h2 className="text-2xl font-bold text-textPrimary">{user?.display_name}</h2>
            <p className="text-textSecondary">{user?.email}</p>
          </div>
        </div>

        <div className="space-y-4">
          <div className="flex items-center gap-3 p-4 bg-surface-elevated rounded-lg">
            <FiUser className="w-5 h-5 text-neuralBlue" />
            <div>
              <div className="text-sm text-textSecondary">Subscription</div>
              <div className="text-textPrimary font-medium capitalize">{user?.subscription}</div>
            </div>
          </div>

          <div className="flex items-center gap-3 p-4 bg-surface-elevated rounded-lg">
            <FiAward className="w-5 h-5 text-creativePurple" />
            <div>
              <div className="text-sm text-textSecondary">Member Since</div>
              <div className="text-textPrimary font-medium">
                {user?.created_at && new Date(user.created_at).toLocaleDateString()}
              </div>
            </div>
          </div>
        </div>

        <div className="mt-6 pt-6 border-t border-surface-elevated">
          <Button
            onClick={handleLogout}
            variant="danger"
            size="lg"
            className="w-full"
          >
            <FiLogOut className="w-5 h-5 mr-2" />
            Sign Out
          </Button>
        </div>
      </Card>
    </div>
  );
}
