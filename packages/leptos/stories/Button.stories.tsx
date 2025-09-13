import type { Meta, StoryObj } from '@storybook/react';
import { Button } from '../button/src/signal_managed';

const meta: Meta<typeof Button> = {
  title: 'Components/Button',
  component: Button,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: 'A versatile button component with signal management integration for Leptos 0.8.8+ applications.',
      },
    },
  },
  tags: ['autodocs'],
  argTypes: {
    variant: {
      control: { type: 'select' },
      options: ['default', 'destructive', 'outline', 'secondary', 'ghost', 'link'],
      description: 'The visual style variant of the button',
    },
    size: {
      control: { type: 'select' },
      options: ['default', 'sm', 'lg', 'icon'],
      description: 'The size of the button',
    },
    disabled: {
      control: { type: 'boolean' },
      description: 'Whether the button is disabled',
    },
    loading: {
      control: { type: 'boolean' },
      description: 'Whether the button is in loading state',
    },
    children: {
      control: { type: 'text' },
      description: 'The content inside the button',
    },
  },
};

export default meta;
type Story = StoryObj<typeof meta>;

// Default button
export const Default: Story = {
  args: {
    children: 'Button',
  },
};

// Variants showcase
export const Variants: Story = {
  render: () => (
    <div className="component-showcase">
      <div className="space-y-4">
        <h3 className="text-lg font-semibold">Button Variants</h3>
        <div className="flex flex-wrap gap-4">
          <Button variant="default">Default</Button>
          <Button variant="destructive">Destructive</Button>
          <Button variant="outline">Outline</Button>
          <Button variant="secondary">Secondary</Button>
          <Button variant="ghost">Ghost</Button>
          <Button variant="link">Link</Button>
        </div>
      </div>
    </div>
  ),
};

// Sizes showcase
export const Sizes: Story = {
  render: () => (
    <div className="component-showcase">
      <div className="space-y-4">
        <h3 className="text-lg font-semibold">Button Sizes</h3>
        <div className="flex items-center gap-4">
          <Button size="sm">Small</Button>
          <Button size="default">Default</Button>
          <Button size="lg">Large</Button>
          <Button size="icon">⚙️</Button>
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
        <h3 className="text-lg font-semibold">Button States</h3>
        <div className="flex flex-wrap gap-4">
          <Button>Normal</Button>
          <Button disabled>Disabled</Button>
          <Button loading>Loading</Button>
          <Button disabled loading>Disabled Loading</Button>
        </div>
      </div>
    </div>
  ),
};

// Interactive example
export const Interactive: Story = {
  render: () => {
    const [count, setCount] = React.useState(0);
    
    return (
      <div className="component-showcase">
        <div className="space-y-4">
          <h3 className="text-lg font-semibold">Interactive Button</h3>
          <p className="text-sm text-gray-600">Click count: {count}</p>
          <div className="flex gap-4">
            <Button onClick={() => setCount(count + 1)}>
              Increment
            </Button>
            <Button variant="outline" onClick={() => setCount(0)}>
              Reset
            </Button>
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
        <h3 className="text-lg font-semibold">Signal-Managed Button</h3>
        <p className="text-sm text-gray-600">
          This button uses Leptos 0.8.8 signal management with ArcRwSignal and ArcMemo
        </p>
        <div className="flex gap-4">
          <Button variant="default">Signal Managed</Button>
          <Button variant="outline" loading>Loading State</Button>
        </div>
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Demonstrates the signal-managed button component with automatic state management and lifecycle handling.',
      },
    },
  },
};
