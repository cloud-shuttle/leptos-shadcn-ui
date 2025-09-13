import type { Meta, StoryObj } from '@storybook/react';
import { SignalManagedInput } from '../input/src/signal_managed';

const meta: Meta<typeof SignalManagedInput> = {
  title: 'Components/Input',
  component: SignalManagedInput,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: 'A versatile input component with signal management integration for Leptos 0.8.8+ applications.',
      },
    },
  },
  tags: ['autodocs'],
  argTypes: {
    type: {
      control: { type: 'select' },
      options: ['text', 'email', 'password', 'number', 'tel', 'url', 'search'],
      description: 'The input type',
    },
    placeholder: {
      control: { type: 'text' },
      description: 'Placeholder text',
    },
    disabled: {
      control: { type: 'boolean' },
      description: 'Whether the input is disabled',
    },
    required: {
      control: { type: 'boolean' },
      description: 'Whether the input is required',
    },
  },
};

export default meta;
type Story = StoryObj<typeof meta>;

// Default input
export const Default: Story = {
  args: {
    placeholder: 'Enter text...',
  },
};

// Input types showcase
export const Types: Story = {
  render: () => (
    <div className="component-showcase">
      <div className="space-y-4">
        <h3 className="text-lg font-semibold">Input Types</h3>
        <div className="grid grid-cols-1 gap-4 max-w-md">
          <div>
            <label className="block text-sm font-medium mb-1">Text</label>
            <SignalManagedInput type="text" placeholder="Enter text..." />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">Email</label>
            <SignalManagedInput type="email" placeholder="Enter email..." />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">Password</label>
            <SignalManagedInput type="password" placeholder="Enter password..." />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">Number</label>
            <SignalManagedInput type="number" placeholder="Enter number..." />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">Search</label>
            <SignalManagedInput type="search" placeholder="Search..." />
          </div>
        </div>
      </div>
    </div>
  ),
};

// States showcase
export const States: Story = {
  render: () => (
    <div className="component-showcase">
      <div className="space-y-4">
        <h3 className="text-lg font-semibold">Input States</h3>
        <div className="grid grid-cols-1 gap-4 max-w-md">
          <div>
            <label className="block text-sm font-medium mb-1">Normal</label>
            <SignalManagedInput placeholder="Normal state" />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">Disabled</label>
            <SignalManagedInput placeholder="Disabled state" disabled />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">Required</label>
            <SignalManagedInput placeholder="Required field" required />
          </div>
        </div>
      </div>
    </div>
  ),
};

// Interactive example
export const Interactive: Story = {
  render: () => {
    const [value, setValue] = React.useState('');
    
    return (
      <div className="component-showcase">
        <div className="space-y-4">
          <h3 className="text-lg font-semibold">Interactive Input</h3>
          <div className="max-w-md">
            <SignalManagedInput
              placeholder="Type something..."
              value={value}
              onChange={(e) => setValue(e.target.value)}
            />
            <p className="text-sm text-gray-600 mt-2">
              Current value: {value || 'Empty'}
            </p>
          </div>
        </div>
      </div>
    );
  },
};

// Validation example
export const Validation: Story = {
  render: () => {
    const [email, setEmail] = React.useState('');
    const [isValid, setIsValid] = React.useState(true);
    
    const validateEmail = (value: string) => {
      const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
      return emailRegex.test(value);
    };
    
    const handleEmailChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      const value = e.target.value;
      setEmail(value);
      setIsValid(validateEmail(value));
    };
    
    return (
      <div className="component-showcase">
        <div className="space-y-4">
          <h3 className="text-lg font-semibold">Input Validation</h3>
          <div className="max-w-md">
            <label className="block text-sm font-medium mb-1">Email Address</label>
            <SignalManagedInput
              type="email"
              placeholder="Enter email address..."
              value={email}
              onChange={handleEmailChange}
              className={!isValid && email ? 'border-red-500' : ''}
            />
            {!isValid && email && (
              <p className="text-sm text-red-600 mt-1">
                Please enter a valid email address
              </p>
            )}
            {isValid && email && (
              <p className="text-sm text-green-600 mt-1">
                Valid email address
              </p>
            )}
          </div>
        </div>
      </div>
    );
  },
};

// Signal management showcase
export const SignalManaged: Story = {
  render: () => (
    <div className="component-showcase">
      <div className="space-y-4">
        <h3 className="text-lg font-semibold">Signal-Managed Input</h3>
        <p className="text-sm text-gray-600">
          This input uses Leptos 0.8.8 signal management with automatic state tracking and validation
        </p>
        <div className="max-w-md">
          <SignalManagedInput
            placeholder="Signal-managed input..."
            type="text"
          />
        </div>
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Demonstrates the signal-managed input component with automatic state management and validation integration.',
      },
    },
  },
};
