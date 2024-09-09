#[doc = "Register `BK4CTRL` reader"]
pub type R = crate::R<BK4CTRL_SPEC>;
#[doc = "Register `BK4CTRL` writer"]
pub type W = crate::W<BK4CTRL_SPEC>;
#[doc = "Field `NWEN` reader - Wait feature enable"]
pub type NWEN_R = crate::BitReader;
#[doc = "Field `NWEN` writer - Wait feature enable"]
pub type NWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - Memory bank enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Memory bank enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV` reader - Memory device type"]
pub type DEV_R = crate::BitReader;
#[doc = "Field `DEV` writer - Memory device type"]
pub type DEV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTMDBW` reader - External memory data bus width"]
pub type EXTMDBW_R = crate::FieldReader;
#[doc = "Field `EXTMDBW` writer - External memory data bus width"]
pub type EXTMDBW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECCEN` reader - ECC enable"]
pub type ECCEN_R = crate::BitReader;
#[doc = "Field `ECCEN` writer - ECC enable"]
pub type ECCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCR` reader - CLE to RE delay"]
pub type TCR_R = crate::FieldReader;
#[doc = "Field `TCR` writer - CLE to RE delay"]
pub type TCR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TAR` reader - ALE to RE delay"]
pub type TAR_R = crate::FieldReader;
#[doc = "Field `TAR` writer - ALE to RE delay"]
pub type TAR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ECCPGS` reader - ECC page size"]
pub type ECCPGS_R = crate::FieldReader;
#[doc = "Field `ECCPGS` writer - ECC page size"]
pub type ECCPGS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    pub fn nwen(&self) -> NWEN_R {
        NWEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Memory bank enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Memory device type"]
    #[inline(always)]
    pub fn dev(&self) -> DEV_R {
        DEV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - External memory data bus width"]
    #[inline(always)]
    pub fn extmdbw(&self) -> EXTMDBW_R {
        EXTMDBW_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - ECC enable"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 9:12 - CLE to RE delay"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - ALE to RE delay"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - ECC page size"]
    #[inline(always)]
    pub fn eccpgs(&self) -> ECCPGS_R {
        ECCPGS_R::new(((self.bits >> 17) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK4CTRL")
            .field("eccpgs", &self.eccpgs())
            .field("tar", &self.tar())
            .field("tcr", &self.tcr())
            .field("eccen", &self.eccen())
            .field("extmdbw", &self.extmdbw())
            .field("dev", &self.dev())
            .field("en", &self.en())
            .field("nwen", &self.nwen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn nwen(&mut self) -> NWEN_W<BK4CTRL_SPEC> {
        NWEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Memory bank enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<BK4CTRL_SPEC> {
        EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Memory device type"]
    #[inline(always)]
    #[must_use]
    pub fn dev(&mut self) -> DEV_W<BK4CTRL_SPEC> {
        DEV_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - External memory data bus width"]
    #[inline(always)]
    #[must_use]
    pub fn extmdbw(&mut self) -> EXTMDBW_W<BK4CTRL_SPEC> {
        EXTMDBW_W::new(self, 4)
    }
    #[doc = "Bit 6 - ECC enable"]
    #[inline(always)]
    #[must_use]
    pub fn eccen(&mut self) -> ECCEN_W<BK4CTRL_SPEC> {
        ECCEN_W::new(self, 6)
    }
    #[doc = "Bits 9:12 - CLE to RE delay"]
    #[inline(always)]
    #[must_use]
    pub fn tcr(&mut self) -> TCR_W<BK4CTRL_SPEC> {
        TCR_W::new(self, 9)
    }
    #[doc = "Bits 13:16 - ALE to RE delay"]
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TAR_W<BK4CTRL_SPEC> {
        TAR_W::new(self, 13)
    }
    #[doc = "Bits 17:19 - ECC page size"]
    #[inline(always)]
    #[must_use]
    pub fn eccpgs(&mut self) -> ECCPGS_W<BK4CTRL_SPEC> {
        ECCPGS_W::new(self, 17)
    }
}
#[doc = "PC Card/NAND Flash control register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`bk4ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk4ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK4CTRL_SPEC;
impl crate::RegisterSpec for BK4CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk4ctrl::R`](R) reader structure"]
impl crate::Readable for BK4CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk4ctrl::W`](W) writer structure"]
impl crate::Writable for BK4CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BK4CTRL to value 0x18"]
impl crate::Resettable for BK4CTRL_SPEC {
    const RESET_VALUE: u32 = 0x18;
}
