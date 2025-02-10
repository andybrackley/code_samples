Running the SBE-Tool

In Cygwin shell navigate to this directory:

/cygdrive/c/Users/andyb/.nuget/packages/sbe-tool/1.23.1.1/tools

Update the sbe-tool.sh to allow the inclusion of included files as follows:

java \
-Dsbe.output.dir="${OUTPUTDIR}" \
-Dsbe.generate.ir="false" \
-Dsbe.xinclude.aware="true" \
-Dsbe.target.language="uk.co.real_logic.sbe.generation.csharp.CSharp" \
-jar "${SBE_JAR}" \
"${SCHEMA}"

Run against some xml shema:

$ sh sbe-tool.sh -d . -s D:/third_party_repos/cpp/simple-binary-encoding/sbe-samples/src/main/resources/example-schema.xml
$ sh sbe-tool.sh -d . -s D:/third_party_repos/cpp/simple-binary-encoding/sbe-samples/src/main/resources/example-extension-schema.xml

Copy across the generated files
