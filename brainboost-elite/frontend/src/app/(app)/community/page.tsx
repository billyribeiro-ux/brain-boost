'use client';

import Card from '@/components/ui/Card';

export default function CommunityPage() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-textPrimary">Community</h1>
        <p className="text-textSecondary mt-1">
          Connect with other cognitive athletes
        </p>
      </div>

      <Card>
        <div className="text-center py-12">
          <p className="text-textSecondary text-lg">
            Community features coming soon...
          </p>
        </div>
      </Card>
    </div>
  );
}
