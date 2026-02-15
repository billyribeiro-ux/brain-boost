'use client';

import { useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import Card from '@/components/ui/Card';
import Button from '@/components/ui/Button';
import Input from '@/components/ui/Input';
import { api } from '@/lib/api';
import { FiSend } from 'react-icons/fi';

export default function CoachPage() {
  const [message, setMessage] = useState('');
  const queryClient = useQueryClient();

  const { data: history } = useQuery({
    queryKey: ['coach-history'],
    queryFn: async () => {
      const response = await api.coach.getHistory();
      return response.data;
    },
  });

  const sendMutation = useMutation({
    mutationFn: (content: string) => api.coach.sendMessage({ content }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['coach-history'] });
      setMessage('');
    },
  });

  const handleSend = () => {
    if (message.trim()) {
      sendMutation.mutate(message);
    }
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-textPrimary">AI Coach</h1>
        <p className="text-textSecondary mt-1">
          Your personal cognitive training assistant
        </p>
      </div>

      <Card className="h-[500px] flex flex-col">
        <div className="flex-1 overflow-y-auto space-y-4 mb-4">
          {history?.map((msg: any) => (
            <div
              key={msg.id}
              className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}
            >
              <div
                className={`max-w-[80%] p-4 rounded-2xl ${
                  msg.role === 'user'
                    ? 'bg-neuralBlue text-white'
                    : 'bg-surface-elevated text-textPrimary'
                }`}
              >
                <p>{msg.content}</p>
                <p className="text-xs opacity-70 mt-2">
                  {new Date(msg.created_at).toLocaleTimeString()}
                </p>
              </div>
            </div>
          ))}
        </div>

        <div className="flex gap-2">
          <Input
            value={message}
            onChange={(e) => setMessage(e.target.value)}
            placeholder="Ask your coach anything..."
            onKeyPress={(e) => e.key === 'Enter' && handleSend()}
          />
          <Button
            onClick={handleSend}
            variant="primary"
            isLoading={sendMutation.isPending}
          >
            <FiSend className="w-5 h-5" />
          </Button>
        </div>
      </Card>
    </div>
  );
}
