using System;
using Microsoft.Win32.SafeHandles;

namespace Bitwarden.SDK
{
    internal class BitwardenClientSafeHandle : SafeHandleZeroOrMinusOneIsInvalid
    {
        public BitwardenClientSafeHandle() : base(true) { }

        public IntPtr Ptr => this.handle;

        protected override bool ReleaseHandle()
        {
            BitwardenClientWrapper.free_mem(handle);
            return true;
        }
    }
}