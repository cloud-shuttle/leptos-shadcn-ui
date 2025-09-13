import type { Meta, StoryObj } from '@storybook/react';
import { SignalManagedButton } from '../button/src/signal_managed';
import { SignalManagedInput } from '../input/src/signal_managed';
import { SignalManagedCard, SignalManagedCardHeader, SignalManagedCardTitle, SignalManagedCardDescription, SignalManagedCardContent, SignalManagedCardFooter } from '../card/src/signal_managed';

const meta: Meta = {
  title: 'Overview/Component Library',
  parameters: {
    layout: 'fullscreen',
    docs: {
      description: {
        component: 'Complete overview of the leptos-shadcn-ui component library with Leptos 0.8.8 signal management integration.',
      },
    },
  },
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof meta>;

// Complete component showcase
export const ComponentShowcase: Story = {
  render: () => (
    <div className="min-h-screen bg-gray-50 p-8">
      <div className="max-w-6xl mx-auto">
        {/* Header */}
        <div className="text-center mb-12">
          <h1 className="text-4xl font-bold text-gray-900 mb-4">
            leptos-shadcn-ui
          </h1>
          <p className="text-xl text-gray-600 mb-2">
            Production-ready ShadCN UI components for Leptos 0.8.8+
          </p>
          <p className="text-sm text-gray-500">
            Built with signal management, memory optimization, and comprehensive testing
          </p>
        </div>

        {/* Features */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
          <div className="bg-white p-6 rounded-lg shadow-sm border">
            <h3 className="text-lg font-semibold mb-2">🚀 Signal Management</h3>
            <p className="text-gray-600 text-sm">
              Built-in ArcRwSignal and ArcMemo integration for optimal performance and state management.
            </p>
          </div>
          <div className="bg-white p-6 rounded-lg shadow-sm border">
            <h3 className="text-lg font-semibold mb-2">🧪 Test-Driven</h3>
            <p className="text-gray-600 text-sm">
              Comprehensive test coverage with TDD methodology and cargo nextest integration.
            </p>
          </div>
          <div className="bg-white p-6 rounded-lg shadow-sm border">
            <h3 className="text-lg font-semibold mb-2">📚 Well Documented</h3>
            <p className="text-gray-600 text-sm">
              Complete API documentation, migration guides, and interactive examples.
            </p>
          </div>
        </div>

        {/* Component Examples */}
        <div className="space-y-12">
          {/* Buttons Section */}
          <section>
            <h2 className="text-2xl font-bold mb-6">Buttons</h2>
            <div className="bg-white p-6 rounded-lg shadow-sm border">
              <div className="space-y-4">
                <div>
                  <h3 className="font-medium mb-2">Variants</h3>
                  <div className="flex flex-wrap gap-3">
                    <SignalManagedButton variant="default">Default</SignalManagedButton>
                    <SignalManagedButton variant="destructive">Destructive</SignalManagedButton>
                    <SignalManagedButton variant="outline">Outline</SignalManagedButton>
                    <SignalManagedButton variant="secondary">Secondary</SignalManagedButton>
                    <SignalManagedButton variant="ghost">Ghost</SignalManagedButton>
                    <SignalManagedButton variant="link">Link</SignalManagedButton>
                  </div>
                </div>
                <div>
                  <h3 className="font-medium mb-2">Sizes</h3>
                  <div className="flex items-center gap-3">
                    <SignalManagedButton size="sm">Small</SignalManagedButton>
                    <SignalManagedButton size="default">Default</SignalManagedButton>
                    <SignalManagedButton size="lg">Large</SignalManagedButton>
                    <SignalManagedButton size="icon">⚙️</SignalManagedButton>
                  </div>
                </div>
                <div>
                  <h3 className="font-medium mb-2">States</h3>
                  <div className="flex gap-3">
                    <SignalManagedButton>Normal</SignalManagedButton>
                    <SignalManagedButton disabled>Disabled</SignalManagedButton>
                    <SignalManagedButton loading>Loading</SignalManagedButton>
                  </div>
                </div>
              </div>
            </div>
          </section>

          {/* Inputs Section */}
          <section>
            <h2 className="text-2xl font-bold mb-6">Inputs</h2>
            <div className="bg-white p-6 rounded-lg shadow-sm border">
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium mb-1">Text Input</label>
                    <SignalManagedInput placeholder="Enter text..." />
                  </div>
                  <div>
                    <label className="block text-sm font-medium mb-1">Email Input</label>
                    <SignalManagedInput type="email" placeholder="Enter email..." />
                  </div>
                  <div>
                    <label className="block text-sm font-medium mb-1">Password Input</label>
                    <SignalManagedInput type="password" placeholder="Enter password..." />
                  </div>
                </div>
                <div className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium mb-1">Number Input</label>
                    <SignalManagedInput type="number" placeholder="Enter number..." />
                  </div>
                  <div>
                    <label className="block text-sm font-medium mb-1">Search Input</label>
                    <SignalManagedInput type="search" placeholder="Search..." />
                  </div>
                  <div>
                    <label className="block text-sm font-medium mb-1">Disabled Input</label>
                    <SignalManagedInput placeholder="Disabled input" disabled />
                  </div>
                </div>
              </div>
            </div>
          </section>

          {/* Cards Section */}
          <section>
            <h2 className="text-2xl font-bold mb-6">Cards</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              <SignalManagedCard>
                <SignalManagedCardHeader>
                  <SignalManagedCardTitle>Basic Card</SignalManagedCardTitle>
                  <SignalManagedCardDescription>
                    A simple card with all sections
                  </SignalManagedCardDescription>
                </SignalManagedCardHeader>
                <SignalManagedCardContent>
                  <p>This is the card content area with some example text.</p>
                </SignalManagedCardContent>
                <SignalManagedCardFooter>
                  <p className="text-sm text-gray-600">Card footer</p>
                </SignalManagedCardFooter>
              </SignalManagedCard>

              <SignalManagedCard>
                <SignalManagedCardHeader>
                  <SignalManagedCardTitle>Content Only</SignalManagedCardTitle>
                  <SignalManagedCardDescription>
                    Card with just content
                  </SignalManagedCardDescription>
                </SignalManagedCardHeader>
                <SignalManagedCardContent>
                  <p>This card only has header and content sections.</p>
                </SignalManagedCardContent>
              </SignalManagedCard>

              <SignalManagedCard>
                <SignalManagedCardContent>
                  <h3 className="font-semibold mb-2">Minimal Card</h3>
                  <p>This card only has content without header or footer.</p>
                </SignalManagedCardContent>
              </SignalManagedCard>
            </div>
          </section>

          {/* Integration Example */}
          <section>
            <h2 className="text-2xl font-bold mb-6">Integration Example</h2>
            <div className="bg-white p-6 rounded-lg shadow-sm border">
              <SignalManagedCard>
                <SignalManagedCardHeader>
                  <SignalManagedCardTitle>Contact Form</SignalManagedCardTitle>
                  <SignalManagedCardDescription>
                    Example form using signal-managed components
                  </SignalManagedCardDescription>
                </SignalManagedCardHeader>
                <SignalManagedCardContent>
                  <div className="space-y-4">
                    <div>
                      <label className="block text-sm font-medium mb-1">Name</label>
                      <SignalManagedInput placeholder="Enter your name..." />
                    </div>
                    <div>
                      <label className="block text-sm font-medium mb-1">Email</label>
                      <SignalManagedInput type="email" placeholder="Enter your email..." />
                    </div>
                    <div>
                      <label className="block text-sm font-medium mb-1">Message</label>
                      <SignalManagedInput placeholder="Enter your message..." />
                    </div>
                  </div>
                </SignalManagedCardContent>
                <SignalManagedCardFooter>
                  <div className="flex gap-3">
                    <SignalManagedButton>Submit</SignalManagedButton>
                    <SignalManagedButton variant="outline">Cancel</SignalManagedButton>
                  </div>
                </SignalManagedCardFooter>
              </SignalManagedCard>
            </div>
          </section>
        </div>

        {/* Footer */}
        <div className="text-center mt-12 pt-8 border-t">
          <p className="text-gray-600">
            Built with ❤️ for the Leptos community
          </p>
          <p className="text-sm text-gray-500 mt-2">
            All components are production-ready with comprehensive testing and documentation
          </p>
        </div>
      </div>
    </div>
  ),
};

// Technical specifications
export const TechnicalSpecs: Story = {
  render: () => (
    <div className="min-h-screen bg-gray-50 p-8">
      <div className="max-w-4xl mx-auto">
        <div className="text-center mb-12">
          <h1 className="text-3xl font-bold text-gray-900 mb-4">
            Technical Specifications
          </h1>
          <p className="text-gray-600">
            Detailed technical information about the component library
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          {/* Signal Management */}
          <div className="bg-white p-6 rounded-lg shadow-sm border">
            <h3 className="text-xl font-semibold mb-4">Signal Management</h3>
            <ul className="space-y-2 text-sm">
              <li>• ArcRwSignal for persistent state</li>
              <li>• ArcMemo for computed values</li>
              <li>• Automatic lifecycle management</li>
              <li>• Memory leak detection</li>
              <li>• Batched updates for performance</li>
            </ul>
          </div>

          {/* Testing */}
          <div className="bg-white p-6 rounded-lg shadow-sm border">
            <h3 className="text-xl font-semibold mb-4">Testing</h3>
            <ul className="space-y-2 text-sm">
              <li>• 100% TDD implementation</li>
              <li>• Comprehensive unit tests</li>
              <li>• Integration test coverage</li>
              <li>• Performance benchmarks</li>
              <li>• Memory management tests</li>
            </ul>
          </div>

          {/* Performance */}
          <div className="bg-white p-6 rounded-lg shadow-sm border">
            <h3 className="text-xl font-semibold mb-4">Performance</h3>
            <ul className="space-y-2 text-sm">
              <li>• Optimized signal updates</li>
              <li>• Memory pressure monitoring</li>
              <li>• Automatic cleanup</li>
              <li>• Efficient re-rendering</li>
              <li>• WASM-optimized</li>
            </ul>
          </div>

          {/* Documentation */}
          <div className="bg-white p-6 rounded-lg shadow-sm border">
            <h3 className="text-xl font-semibold mb-4">Documentation</h3>
            <ul className="space-y-2 text-sm">
              <li>• Complete API documentation</li>
              <li>• Migration guides</li>
              <li>• Interactive examples</li>
              <li>• Storybook integration</li>
              <li>• Best practices guide</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  ),
};
