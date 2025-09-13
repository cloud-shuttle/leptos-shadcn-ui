import type { Meta, StoryObj } from '@storybook/react';
import { SignalManagedCard, SignalManagedCardHeader, SignalManagedCardTitle, SignalManagedCardDescription, SignalManagedCardContent, SignalManagedCardFooter } from '../card/src/signal_managed';

const meta: Meta<typeof SignalManagedCard> = {
  title: 'Components/Card',
  component: SignalManagedCard,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: 'A versatile card component with signal management integration for Leptos 0.8.8+ applications.',
      },
    },
  },
  tags: ['autodocs'],
  argTypes: {
    disabled: {
      control: { type: 'boolean' },
      description: 'Whether the card is disabled',
    },
  },
};

export default meta;
type Story = StoryObj<typeof meta>;

// Default card
export const Default: Story = {
  render: () => (
    <SignalManagedCard className="w-80">
      <SignalManagedCardHeader>
        <SignalManagedCardTitle>Card Title</SignalManagedCardTitle>
        <SignalManagedCardDescription>
          Card description goes here
        </SignalManagedCardDescription>
      </SignalManagedCardHeader>
      <SignalManagedCardContent>
        <p>This is the card content area.</p>
      </SignalManagedCardContent>
      <SignalManagedCardFooter>
        <p>Card footer content</p>
      </SignalManagedCardFooter>
    </SignalManagedCard>
  ),
};

// Card variants
export const Variants: Story = {
  render: () => (
    <div className="component-showcase">
      <div className="space-y-6">
        <h3 className="text-lg font-semibold">Card Variants</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* Basic Card */}
          <SignalManagedCard>
            <SignalManagedCardHeader>
              <SignalManagedCardTitle>Basic Card</SignalManagedCardTitle>
              <SignalManagedCardDescription>
                A simple card with header, content, and footer
              </SignalManagedCardDescription>
            </SignalManagedCardHeader>
            <SignalManagedCardContent>
              <p>This is a basic card implementation.</p>
            </SignalManagedCardContent>
            <SignalManagedCardFooter>
              <p className="text-sm text-gray-600">Footer</p>
            </SignalManagedCardFooter>
          </SignalManagedCard>

          {/* Content Only Card */}
          <SignalManagedCard>
            <SignalManagedCardContent>
              <h3 className="font-semibold mb-2">Content Only</h3>
              <p>This card only has content without header or footer.</p>
            </SignalManagedCardContent>
          </SignalManagedCard>

          {/* Header Only Card */}
          <SignalManagedCard>
            <SignalManagedCardHeader>
              <SignalManagedCardTitle>Header Only</SignalManagedCardTitle>
              <SignalManagedCardDescription>
                This card only has a header section
              </SignalManagedCardDescription>
            </SignalManagedCardHeader>
          </SignalManagedCard>

          {/* Footer Only Card */}
          <SignalManagedCard>
            <SignalManagedCardContent>
              <p>This card has content and footer but no header.</p>
            </SignalManagedCardContent>
            <SignalManagedCardFooter>
              <p className="text-sm text-gray-600">Footer only</p>
            </SignalManagedCardFooter>
          </SignalManagedCard>
        </div>
      </div>
    </div>
  ),
};

// Card with interactive content
export const Interactive: Story = {
  render: () => {
    const [count, setCount] = React.useState(0);
    
    return (
      <div className="component-showcase">
        <div className="space-y-4">
          <h3 className="text-lg font-semibold">Interactive Card</h3>
          <SignalManagedCard className="w-80">
            <SignalManagedCardHeader>
              <SignalManagedCardTitle>Counter Card</SignalManagedCardTitle>
              <SignalManagedCardDescription>
                An interactive card with state management
              </SignalManagedCardDescription>
            </SignalManagedCardHeader>
            <SignalManagedCardContent>
              <div className="space-y-4">
                <p>Current count: <span className="font-bold">{count}</span></p>
                <div className="flex gap-2">
                  <button
                    onClick={() => setCount(count - 1)}
                    className="px-3 py-1 bg-gray-200 rounded hover:bg-gray-300"
                  >
                    -
                  </button>
                  <button
                    onClick={() => setCount(count + 1)}
                    className="px-3 py-1 bg-gray-200 rounded hover:bg-gray-300"
                  >
                    +
                  </button>
                  <button
                    onClick={() => setCount(0)}
                    className="px-3 py-1 bg-gray-200 rounded hover:bg-gray-300"
                  >
                    Reset
                  </button>
                </div>
              </div>
            </SignalManagedCardContent>
            <SignalManagedCardFooter>
              <p className="text-sm text-gray-600">
                Click the buttons to interact
              </p>
            </SignalManagedCardFooter>
          </SignalManagedCard>
        </div>
      </div>
    );
  },
};

// Card states
export const States: Story = {
  render: () => (
    <div className="component-showcase">
      <div className="space-y-6">
        <h3 className="text-lg font-semibold">Card States</h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* Normal State */}
          <SignalManagedCard>
            <SignalManagedCardHeader>
              <SignalManagedCardTitle>Normal State</SignalManagedCardTitle>
              <SignalManagedCardDescription>
                This card is in its normal state
              </SignalManagedCardDescription>
            </SignalManagedCardHeader>
            <SignalManagedCardContent>
              <p>Content is fully interactive.</p>
            </SignalManagedCardContent>
          </SignalManagedCard>

          {/* Disabled State */}
          <SignalManagedCard disabled>
            <SignalManagedCardHeader>
              <SignalManagedCardTitle>Disabled State</SignalManagedCardTitle>
              <SignalManagedCardDescription>
                This card is disabled
              </SignalManagedCardDescription>
            </SignalManagedCardHeader>
            <SignalManagedCardContent>
              <p>Content is not interactive.</p>
            </SignalManagedCardContent>
          </SignalManagedCard>
        </div>
      </div>
    </div>
  ),
};

// Card layout examples
export const Layouts: Story = {
  render: () => (
    <div className="component-showcase">
      <div className="space-y-6">
        <h3 className="text-lg font-semibold">Card Layouts</h3>
        
        {/* Horizontal Layout */}
        <div>
          <h4 className="font-medium mb-3">Horizontal Layout</h4>
          <SignalManagedCard className="flex flex-row">
            <SignalManagedCardContent className="flex-1">
              <h3 className="font-semibold mb-2">Horizontal Card</h3>
              <p>This card uses a horizontal layout with flexbox.</p>
            </SignalManagedCardContent>
            <SignalManagedCardFooter className="flex items-center">
              <span className="text-sm text-gray-600">Side footer</span>
            </SignalManagedCardFooter>
          </SignalManagedCard>
        </div>

        {/* Grid Layout */}
        <div>
          <h4 className="font-medium mb-3">Grid Layout</h4>
          <div className="grid grid-cols-3 gap-4">
            <SignalManagedCard>
              <SignalManagedCardContent>
                <h3 className="font-semibold mb-2">Grid Item 1</h3>
                <p>First grid item</p>
              </SignalManagedCardContent>
            </SignalManagedCard>
            <SignalManagedCard>
              <SignalManagedCardContent>
                <h3 className="font-semibold mb-2">Grid Item 2</h3>
                <p>Second grid item</p>
              </SignalManagedCardContent>
            </SignalManagedCard>
            <SignalManagedCard>
              <SignalManagedCardContent>
                <h3 className="font-semibold mb-2">Grid Item 3</h3>
                <p>Third grid item</p>
              </SignalManagedCardContent>
            </SignalManagedCard>
          </div>
        </div>
      </div>
    </div>
  ),
};

// Signal management showcase
export const SignalManaged: Story = {
  render: () => (
    <div className="component-showcase">
      <div className="space-y-4">
        <h3 className="text-lg font-semibold">Signal-Managed Card</h3>
        <p className="text-sm text-gray-600">
          This card uses Leptos 0.8.8 signal management with automatic state tracking and lifecycle management
        </p>
        <SignalManagedCard className="w-80">
          <SignalManagedCardHeader>
            <SignalManagedCardTitle>Signal-Managed Card</SignalManagedCardTitle>
            <SignalManagedCardDescription>
              Demonstrates signal management integration
            </SignalManagedCardDescription>
          </SignalManagedCardHeader>
          <SignalManagedCardContent>
            <p>This card automatically manages its state using ArcRwSignal and ArcMemo.</p>
          </SignalManagedCardContent>
          <SignalManagedCardFooter>
            <p className="text-sm text-gray-600">Signal-managed footer</p>
          </SignalManagedCardFooter>
        </SignalManagedCard>
      </div>
    </div>
  ),
  parameters: {
    docs: {
      description: {
        story: 'Demonstrates the signal-managed card component with automatic state management and lifecycle handling.',
      },
    },
  },
};
