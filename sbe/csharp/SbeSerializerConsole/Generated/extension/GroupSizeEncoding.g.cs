// <auto-generated>
//     Generated SBE (Simple Binary Encoding) message codec
// </auto-generated>

#pragma warning disable 1591 // disable warning on missing comments
using System;
using System.Text;
using Org.SbeTool.Sbe.Dll;

namespace Extension
{
    /// <summary>
    /// Repeating group dimensions.
    /// </summary>
    public sealed partial class GroupSizeEncoding
    {
        public const ushort SbeSchemaId = (ushort)1;
        public const ushort SbeSchemaVersion = (ushort)1;
        public const int Size = 4;

        private DirectBuffer _buffer;
        private int _offset;
        private int _actingVersion;

        public void Wrap(DirectBuffer buffer, int offset, int actingVersion)
        {
            _offset = offset;
            _actingVersion = actingVersion;
            _buffer = buffer;
        }


        public const ushort BlockLengthNullValue = (ushort)65535;
        public const ushort BlockLengthMinValue = (ushort)0;
        public const ushort BlockLengthMaxValue = (ushort)65534;

        public ushort BlockLength
        {
            get
            {
                return _buffer.Uint16GetLittleEndian(_offset + 0);
            }
            set
            {
                _buffer.Uint16PutLittleEndian(_offset + 0, value);
            }
        }


        public const ushort NumInGroupNullValue = (ushort)65535;
        public const ushort NumInGroupMinValue = (ushort)0;
        public const ushort NumInGroupMaxValue = (ushort)65534;

        public ushort NumInGroup
        {
            get
            {
                return _buffer.Uint16GetLittleEndian(_offset + 2);
            }
            set
            {
                _buffer.Uint16PutLittleEndian(_offset + 2, value);
            }
        }


        public override string ToString()
        {
            var sb = new StringBuilder(100);
            this.BuildString(sb);
            return sb.ToString();
        }

        internal void BuildString(StringBuilder builder)
        {
            if (_buffer == null)
            {
                return;
            }

            builder.Append('(');
            builder.Append("BlockLength=");
            builder.Append(this.BlockLength);
            builder.Append('|');
            builder.Append("NumInGroup=");
            builder.Append(this.NumInGroup);
            builder.Append(')');

        }
    }
}
