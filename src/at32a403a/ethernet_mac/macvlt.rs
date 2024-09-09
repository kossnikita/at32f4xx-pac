#[doc = "Register `MACVLT` reader"]
pub type R = crate::R<MACVLT_SPEC>;
#[doc = "Register `MACVLT` writer"]
pub type W = crate::W<MACVLT_SPEC>;
#[doc = "Field `VTI` reader - VLAN tag identifier (for receive frames)"]
pub type VTI_R = crate::FieldReader<u16>;
#[doc = "Field `VTI` writer - VLAN tag identifier (for receive frames)"]
pub type VTI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ETV` reader - Enable 12-bit VLAN tag comparison"]
pub type ETV_R = crate::BitReader;
#[doc = "Field `ETV` writer - Enable 12-bit VLAN tag comparison"]
pub type ETV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vti(&self) -> VTI_R {
        VTI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACVLT")
            .field("vti", &self.vti())
            .field("etv", &self.etv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    #[must_use]
    pub fn vti(&mut self) -> VTI_W<MACVLT_SPEC> {
        VTI_W::new(self, 0)
    }
    #[doc = "Bit 16 - Enable 12-bit VLAN tag comparison"]
    #[inline(always)]
    #[must_use]
    pub fn etv(&mut self) -> ETV_W<MACVLT_SPEC> {
        ETV_W::new(self, 16)
    }
}
#[doc = "Ethernet MAC VLAN tag register\n\nYou can [`read`](crate::Reg::read) this register and get [`macvlt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvlt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACVLT_SPEC;
impl crate::RegisterSpec for MACVLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvlt::R`](R) reader structure"]
impl crate::Readable for MACVLT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macvlt::W`](W) writer structure"]
impl crate::Writable for MACVLT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACVLT to value 0"]
impl crate::Resettable for MACVLT_SPEC {
    const RESET_VALUE: u32 = 0;
}
