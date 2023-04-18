using System;

namespace Bitwarden.SDK
{
    public class BitwardenClient  : IDisposable
    {
        private readonly BitwardenClientSafeHandle handle;

        public BitwardenClient()
        {
            handle = BitwardenClientWrapper.init("");
        }
        
        protected virtual void Dispose(bool disposing)
        {
            if (handle != null && !handle.IsInvalid)
                handle.Dispose();
        }
        
        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }
    }
}