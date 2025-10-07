import {
  reactExtension,
  Banner,
  useExtensionApi,
} from '@shopify/ui-extensions-react/checkout';
export default reactExtension(
  'purchase.checkout.block.render',
  () => <CheckoutExtension />,
);
function CheckoutExtension() {
  const { lines } = useExtensionApi();
  let showNotice = lines.current.some(item => item.merchandise.title.includes('NoteBook'))
  if (showNotice) {
    return (
      <Banner
        status="warning"
        title="NoteBook do not ship out VietNam."
      />
    );
  }
}
